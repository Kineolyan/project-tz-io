package com.kineolyan.tzio.v1.multi;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Base64;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.Queue;
import java.util.Set;
import java.util.StringJoiner;
import java.util.concurrent.Semaphore;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * Execution of a program, running each node in its own sub-kernel.
 * <p>
 *   The host owns the slots connecting the nodes. At each tick, it reads the I/O reported by
 *   every node, updates the slots, then sends to every node at once the resources made
 *   available.
 * </p>
 */
final class HostExecution implements AutoCloseable {

	/** Class logger */
	private static final Logger logger = Logger.getLogger(HostExecution.class.getName());

	/** Nodes of the program */
	private final List<MultiTzEnv.NodeSpec> nodes;
	/** Writer of each slot, null if none */
	private final MultiTzEnv.Endpoint[] writers;
	/** Indexes of the slots fed by external data */
	private final int[] inputs;
	/** Indexes of the slots read to produce data */
	private final int[] outputs;
	/** Maximal time to wait for a message during an idle tick */
	private final long tickWaitMs;
	/** Processes running the nodes, in the order of {@link #nodes} */
	private final List<NodeProcess> processes;
	/** Signal released every time a process writes a line */
	private final Semaphore activity;

	/** Flags marking slots holding a value */
	private final boolean[] full;
	/** Flags marking slots whose value was sent to their reader */
	private final boolean[] delivered;
	/** Values of the slots */
	private final int[] values;
	/** Pending values of external inputs */
	private final List<Queue<Integer>> inputQueues;
	/** Outputs freed since the last sync, for each node */
	private final List<List<Integer>> frees;

	private HostExecution(
			final List<MultiTzEnv.NodeSpec> nodes,
			final int slotCount,
			final MultiTzEnv.Endpoint[] writers,
			final int[] inputs,
			final int[] outputs,
			final long tickWaitMs) {
		this.nodes = nodes;
		this.writers = writers;
		this.inputs = inputs;
		this.outputs = outputs;
		this.tickWaitMs = tickWaitMs;
		this.processes = new ArrayList<>(nodes.size());
		this.activity = new Semaphore(0);

		this.full = new boolean[slotCount];
		this.delivered = new boolean[slotCount];
		this.values = new int[slotCount];
		this.inputQueues = new ArrayList<>(inputs.length);
		for (int i = 0; i < inputs.length; i += 1) {
			this.inputQueues.add(new ArrayDeque<>());
		}
		this.frees = new ArrayList<>(nodes.size());
		for (int i = 0; i < nodes.size(); i += 1) {
			this.frees.add(new ArrayList<>());
		}
	}

	/**
	 * Starts the execution of a program.
	 * <p>
	 *   This starts all sub-kernels, loads their program and waits for all of them to be
	 *   ready before starting them.
	 * </p>
	 * @param config configuration of the kernels
	 * @param nodes nodes of the program
	 * @param slotCount number of slots
	 * @param writers writer of each slot
	 * @param inputs slots fed by external data
	 * @param outputs slots read to produce data
	 * @param traced true to record the stderr of the sub-kernels
	 * @param tickWaitMs maximal time to wait for a message during an idle tick
	 * @return the started execution
	 */
	static HostExecution start(
			final KernelConfig config,
			final List<MultiTzEnv.NodeSpec> nodes,
			final int slotCount,
			final MultiTzEnv.Endpoint[] writers,
			final int[] inputs,
			final int[] outputs,
			final boolean traced,
			final long tickWaitMs) {
		final HostExecution execution = new HostExecution(nodes, slotCount, writers, inputs, outputs, tickWaitMs);
		try {
			for (final MultiTzEnv.NodeSpec node : nodes) {
				final String command = config.commandFor(node.name());
				execution.processes.add(NodeProcess.start(
						node.name(),
						command,
						config.directory(),
						traced,
						execution.activity::release));
			}
			for (int i = 0; i < nodes.size(); i += 1) {
				final String bytecode = Base64.getEncoder().encodeToString(nodes.get(i).bytecode());
				execution.processes.get(i).send("LOAD " + bytecode);
			}
			execution.awaitReady();
			execution.processes.forEach(process -> process.send("START"));
		} catch (final RuntimeException e) {
			execution.close();
			throw e;
		}
		return execution;
	}

	private void awaitReady() {
		final Set<NodeProcess> pending = new HashSet<>(this.processes);
		while (!pending.isEmpty()) {
			for (final var it = pending.iterator(); it.hasNext(); ) {
				final NodeProcess process = it.next();
				final String line = process.poll();
				if ("READY".equals(line)) {
					it.remove();
				} else if (line != null) {
					throw protocolError(process, line);
				} else if (process.hasEnded()) {
					throw new IllegalStateException(process.describeEnd());
				}
			}
			if (!pending.isEmpty()) {
				waitForActivity();
			}
		}
	}

	/**
	 * Feeds the execution with external data.
	 * @param input values for each external input
	 */
	void consume(final OptionalInt[] input) {
		for (int i = 0, end = Math.min(input.length, this.inputQueues.size()); i < end; i += 1) {
			if (input[i].isPresent()) {
				this.inputQueues.get(i).offer(input[i].getAsInt());
			}
		}
	}

	/**
	 * Executes a tick of the host.
	 * @return the values produced on the external outputs, if any
	 */
	Optional<OptionalInt[]> tick() {
		if (this.activity.drainPermits() == 0) {
			waitForActivity();
		}

		readNodes();
		feedInputs();
		final Optional<OptionalInt[]> produced = collectOutputs();
		syncNodes();

		return produced;
	}

	private void waitForActivity() {
		try {
			this.activity.tryAcquire(this.tickWaitMs, TimeUnit.MILLISECONDS);
		} catch (final InterruptedException e) {
			Thread.currentThread().interrupt();
			throw new IllegalStateException("Execution interrupted", e);
		}
		this.activity.drainPermits();
	}

	private void readNodes() {
		for (int i = 0; i < this.processes.size(); i += 1) {
			final NodeProcess process = this.processes.get(i);
			String line;
			while ((line = process.poll()) != null) {
				applyIo(i, process, line);
			}
			if (process.hasEnded()) {
				throw new IllegalStateException(process.describeEnd());
			}
		}
	}

	/**
	 * Applies an {@code IO <consumed> <produced>} message to the slots.
	 */
	private void applyIo(final int nodeIdx, final NodeProcess process, final String line) {
		final String[] parts = line.split(" ");
		if (parts.length != 3 || !parts[0].equals("IO")) {
			throw protocolError(process, line);
		}
		final MultiTzEnv.NodeSpec node = this.nodes.get(nodeIdx);
		try {
			if (!parts[1].equals("-")) {
				final int slot = node.inputs()[Integer.parseInt(parts[1])];
				if (!this.full[slot] || !this.delivered[slot]) {
					throw protocolError(process, line);
				}
				release(slot);
			}
			if (!parts[2].equals("-")) {
				final int separator = parts[2].indexOf(':');
				final int slot = node.outputs()[Integer.parseInt(parts[2].substring(0, separator))];
				if (this.full[slot]) {
					throw protocolError(process, line);
				}
				fill(slot, Integer.parseInt(parts[2].substring(separator + 1)));
			}
		} catch (final NumberFormatException | IndexOutOfBoundsException e) {
			final IllegalStateException error = protocolError(process, line);
			error.initCause(e);
			throw error;
		}
	}

	private void feedInputs() {
		for (int i = 0; i < this.inputs.length; i += 1) {
			final int slot = this.inputs[i];
			final Queue<Integer> queue = this.inputQueues.get(i);
			if (!this.full[slot] && !queue.isEmpty()) {
				fill(slot, queue.poll());
			}
		}
	}

	private Optional<OptionalInt[]> collectOutputs() {
		final OptionalInt[] produced = new OptionalInt[this.outputs.length];
		boolean hasOutput = false;
		for (int i = 0; i < this.outputs.length; i += 1) {
			final int slot = this.outputs[i];
			if (this.full[slot]) {
				produced[i] = OptionalInt.of(this.values[slot]);
				release(slot);
				hasOutput = true;
			} else {
				produced[i] = OptionalInt.empty();
			}
		}
		return hasOutput ? Optional.of(produced) : Optional.empty();
	}

	/**
	 * Sends to each node, in one message, its new input values and its freed outputs.
	 */
	private void syncNodes() {
		for (int i = 0; i < this.nodes.size(); i += 1) {
			final int[] nodeInputs = this.nodes.get(i).inputs();
			final StringJoiner newValues = new StringJoiner(",");
			newValues.setEmptyValue("-");
			for (int input = 0; input < nodeInputs.length; input += 1) {
				final int slot = nodeInputs[input];
				if (this.full[slot] && !this.delivered[slot]) {
					this.delivered[slot] = true;
					newValues.add(input + ":" + this.values[slot]);
				}
			}

			final List<Integer> freed = this.frees.get(i);
			if (newValues.length() > 1 || !freed.isEmpty()) {
				final StringJoiner freedOutputs = new StringJoiner(",");
				freedOutputs.setEmptyValue("-");
				freed.forEach(output -> freedOutputs.add(output.toString()));
				freed.clear();

				final String message = "SYNC " + newValues + " " + freedOutputs;
				if (logger.isLoggable(Level.FINE)) {
					logger.fine(this.nodes.get(i).name() + " <- " + message);
				}
				this.processes.get(i).send(message);
			}
		}
	}

	private void fill(final int slot, final int value) {
		this.full[slot] = true;
		this.delivered[slot] = false;
		this.values[slot] = value;
	}

	/**
	 * Empties a slot, notifying its writer.
	 */
	private void release(final int slot) {
		this.full[slot] = false;
		this.delivered[slot] = false;
		final MultiTzEnv.Endpoint writer = this.writers[slot];
		if (writer != null && !writer.isExternal()) {
			this.frees.get(writer.node()).add(writer.index());
		}
	}

	private static IllegalStateException protocolError(final NodeProcess process, final String line) {
		return new IllegalStateException("Unexpected message from node " + process.name() + ": " + line);
	}

	/**
	 * Gets the lines written on stderr by each node.
	 * @return the traces indexed by node names
	 */
	Map<String, List<String>> trace() {
		final Map<String, List<String>> trace = new LinkedHashMap<>();
		this.processes.forEach(process -> trace.put(process.name(), process.trace()));
		return trace;
	}

	@Override
	public void close() {
		this.processes.forEach(NodeProcess::close);
	}

}
