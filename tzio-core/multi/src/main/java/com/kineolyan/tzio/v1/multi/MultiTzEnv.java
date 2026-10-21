package com.kineolyan.tzio.v1.multi;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.ops.OperationType;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.OptionalInt;
import java.util.PrimitiveIterator;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * TZ-IO environment running each node in a separate process.
 * <p>
 *   The kernel running each node is selected by the {@link KernelConfig}. See PROTOCOL.md for the
 *   communication between the host and the kernels.
 * </p>
 */
public class MultiTzEnv implements TzEnv {

	/** System property enabling the recording of the kernels stderr */
	public static final String TRACE_PROPERTY = "tzio.multi.trace";
	/** System property defining the maximal time to wait for a message during an idle tick */
	public static final String TICK_WAIT_PROPERTY = "tzio.multi.tickWaitMs";
	/** Default value of {@link #TICK_WAIT_PROPERTY} */
	public static final long DEFAULT_TICK_WAIT_MS = 20;

	/** Separator of values in the lines read from the standard input */
	private static final String SPLIT_CHAR = ";";

	/** Supplier of the kernel configuration, called when the program starts */
	private final Supplier<KernelConfig> config;
	/** Nodes of the program */
	private final List<NodeSpec> nodes;
	/** Total count of slots */
	private int slotCount;
	/** Indexes of the slots fed by external data */
	private int[] inputs;
	/** Indexes of the slots read to produce data */
	private int[] outputs;
	/** Writer of each slot */
	private Endpoint[] writers;
	/** Reader of each slot */
	private Endpoint[] readers;
	/** Traces of the last execution */
	private Map<String, List<String>> trace;

	/**
	 * Constructor.
	 * @param config supplier of the kernel configuration, called when the program starts
	 */
	public MultiTzEnv(final Supplier<KernelConfig> config) {
		this.config = config;
		this.nodes = new ArrayList<>();
		this.trace = Map.of();
	}

	@Override
	public MultiTzEnv withSlots(final int slotCount, final int[] inputs, final int[] outputs) {
		this.slotCount = slotCount;
		this.inputs = inputs.clone();
		this.outputs = outputs.clone();
		this.writers = new Endpoint[slotCount];
		this.readers = new Endpoint[slotCount];
		for (int i = 0; i < inputs.length; i += 1) {
			register(this.writers, inputs[i], Endpoint.external(i), "Input");
		}
		for (int i = 0; i < outputs.length; i += 1) {
			register(this.readers, outputs[i], Endpoint.external(i), "Output");
		}
		return this;
	}

	@Override
	public MultiTzEnv addNode(
			final String name,
			final int memorySize,
			final int[] inputs,
			final int[] outputs,
			final List<OperationType> operations) {
		if (this.writers == null) {
			throw new IllegalStateException("Slots must be configured before adding nodes");
		}
		if (this.nodes.stream().anyMatch(node -> node.name().equals(name))) {
			throw new IllegalStateException("Existing node registered under " + name);
		}

		final int nodeIdx = this.nodes.size();
		for (int i = 0; i < inputs.length; i += 1) {
			register(this.readers, inputs[i], new Endpoint(nodeIdx, i), "Node " + name + " input");
		}
		for (int i = 0; i < outputs.length; i += 1) {
			register(this.writers, outputs[i], new Endpoint(nodeIdx, i), "Node " + name + " output");
		}
		final byte[] bytecode = BytecodeEncoder.encode(inputs.length, outputs.length, memorySize, operations);
		this.nodes.add(new NodeSpec(name, inputs.clone(), outputs.clone(), bytecode));
		return this;
	}

	/**
	 * Registers the reader or the writer of a slot, checking that a slot has at most one of each.
	 */
	private void register(final Endpoint[] endpoints, final int slot, final Endpoint endpoint, final String description) {
		if (slot < 0 || slot >= this.slotCount) {
			throw new IllegalArgumentException(description + " uses unknown slot " + slot);
		}
		if (endpoints[slot] != null) {
			throw new IllegalArgumentException(description + " shares slot " + slot + " with another "
					+ (endpoints == this.writers ? "writer" : "reader"));
		}
		endpoints[slot] = endpoint;
	}

	@Override
	public void runFromSystem(final String[] args) {
		final BlockingQueue<OptionalInt[]> pendingInputs = new LinkedBlockingQueue<>();
		final Thread inputThread = new Thread(() -> readSystemInputs(pendingInputs), "input-thread");
		inputThread.setDaemon(true);
		inputThread.start();

		try (HostExecution execution = start()) {
			System.out.printf("System up. Waiting for inputs (%d):%n", this.inputs.length);
			while (true) {
				OptionalInt[] input;
				while ((input = pendingInputs.poll()) != null) {
					execution.consume(input);
				}
				execution.tick().ifPresent(output -> System.out.println(Stream.of(output)
						.map(o -> o.isPresent() ? String.valueOf(o.getAsInt()) : "")
						.collect(Collectors.joining(SPLIT_CHAR, "> ", ""))));
			}
		}
	}

	private static void readSystemInputs(final BlockingQueue<OptionalInt[]> pendingInputs) {
		try (BufferedReader reader = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8))) {
			String line;
			while ((line = reader.readLine()) != null) {
				pendingInputs.offer(Stream.of(line.split("\\s*" + SPLIT_CHAR + "\\s*"))
						.map(value -> value.isBlank() ? OptionalInt.empty() : OptionalInt.of(Integer.parseInt(value.strip())))
						.toArray(OptionalInt[]::new));
			}
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}

	@Override
	public Stream<OptionalInt[]> runOn(final IntStream[] inputs, final int cycles) {
		final List<OptionalInt[]> results = new ArrayList<>();
		final HostExecution execution = start();
		try (execution) {
			feedInputs(execution, inputs);
			for (int i = 0; i < cycles; i += 1) {
				execution.tick().ifPresent(results::add);
			}
		} finally {
			this.trace = execution.trace();
		}
		return results.stream();
	}

	private static void feedInputs(final HostExecution execution, final IntStream[] inputs) {
		final List<PrimitiveIterator.OfInt> iterators = Stream.of(inputs)
				.map(IntStream::iterator)
				.toList();
		while (iterators.stream().anyMatch(PrimitiveIterator.OfInt::hasNext)) {
			execution.consume(iterators.stream()
					.map(it -> it.hasNext() ? OptionalInt.of(it.nextInt()) : OptionalInt.empty())
					.toArray(OptionalInt[]::new));
		}
	}

	private HostExecution start() {
		if (this.writers == null) {
			throw new IllegalStateException("Slots must be configured before running");
		}
		return HostExecution.start(
				this.config.get(),
				List.copyOf(this.nodes),
				this.slotCount,
				this.writers.clone(),
				this.inputs,
				this.outputs,
				Boolean.getBoolean(TRACE_PROPERTY),
				Long.getLong(TICK_WAIT_PROPERTY, DEFAULT_TICK_WAIT_MS));
	}

	/**
	 * Gets the lines written on stderr by each node during the last execution.
	 * <p>
	 *   Lines are only recorded when the system property {@value #TRACE_PROPERTY} is {@code true}.
	 * </p>
	 * @return the traces indexed by node names
	 */
	public Map<String, List<String>> trace() {
		return this.trace;
	}

	/**
	 * Definition of a node.
	 * @param name name of the node
	 * @param inputs slots read by the node
	 * @param outputs slots written by the node
	 * @param bytecode program of the node
	 */
	record NodeSpec(String name, int[] inputs, int[] outputs, byte[] bytecode) {}

	/**
	 * Reader or writer of a slot.
	 * @param node index of the node, or -1 for the outside world
	 * @param index index of the input or output in the node, or in the environment
	 */
	record Endpoint(int node, int index) {

		static Endpoint external(final int index) {
			return new Endpoint(-1, index);
		}

		boolean isExternal() {
			return this.node < 0;
		}

	}

}
