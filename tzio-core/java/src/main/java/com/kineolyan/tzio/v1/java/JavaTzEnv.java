package com.kineolyan.tzio.v1.java;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.ops.OperationType;
import com.kineolyan.tzio.v1.java.execs.StaticExecutor;
import com.kineolyan.tzio.v1.java.execs.SystemExecutor;
import com.kineolyan.tzio.v1.java.ops.Operation;
import com.kineolyan.tzio.v1.java.slot.DataSlot;
import com.kineolyan.tzio.v1.java.slot.InputQueueSlot;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;

import java.util.*;
import java.util.function.Consumer;
import java.util.function.IntFunction;
import java.util.logging.Level;
import java.util.logging.Logger;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * Representation of a whole TZ-IO environment.
 * <p>
 *   This contains the nodes in the environment, the operations to run on each node.
 *   It connects nodes between each other, as well as nodes to the outside world.
 * </p>
 */
public class JavaTzEnv implements TzEnv {

	/** Class logger */
	private static final Logger logger = Logger.getLogger(JavaTzEnv.class.getName());

	private final TzAdapter adapter;

	/** Map of node executions indexed by node names */
	private final Map<String, NodeExecution> nodes;
	/**
	 * All slots defined in the environment.
	 * <p>
	 *   For java-generic reasons, this array holds {@link TransactionalElement}.
	 * </p>
	 */
	private TransactionalElement[] slots;
	/**	Input slots fed by external data */
	private InputQueueSlot[] inputs;
	/** Output slots to read to produce data */
	private DataSlot[] outputs;
	/** Entity consuming data produced by this TZ-IO program */
	private Consumer<OptionalInt[]> consumer;

	/**
	 * Constructor
	 */
	public JavaTzEnv() {
		this.adapter = new TzAdapter();
		this.nodes = new HashMap<>();
		this.consumer = values -> {};
	}

	/**
	 * Configure the slots existing in this environment.
	 * @param slotCount total count of slots
	 * @param inputs indexes of slots to use for external inputs
	 * @param outputs indexes of slots to read to produce data
	 * @return this
	 */
	public JavaTzEnv withSlots(
		final int slotCount,
		final int[] inputs,
		final int[] outputs) {
		final TransactionalElement[] slots = IntStream.range(0, slotCount)
			.mapToObj(i -> new DataSlot())
			.toArray(TransactionalElement[]::new);
		this.inputs = IntStream.of(inputs)
			.mapToObj(i -> {
				final InputQueueSlot inputSlot = new InputQueueSlot();
				// Replace the default slot
				slots[i] = inputSlot;
				return inputSlot;
			})
			.toArray(InputQueueSlot[]::new);
		this.outputs = getSlots(slots, outputs, DataSlot[]::new);
		this.slots = slots;

		if (logger.isLoggable(Level.FINE)) {
			logger.fine(String.format(
				"Environment configured with %d slots.%nInputs are %s%nOutputs are %s",
				slotCount,
				Arrays.toString(inputs),
				Arrays.toString(outputs)));
		}

		return this;
	}

	@Override
	public TzEnv addNode(
			final String name,
			final int memorySize,
			final int[] inputs,
			final int[] outputs,
			final List<OperationType> operations) {
		return addImplNode(
				name,
				memorySize,
				inputs,
				outputs,
				operations.stream()
					.map(this.adapter::convert)
					.collect(Collectors.toList()));
	}

	/**
	 * Adds a node in this environment.
	 * @param name name of the node
	 * @param memorySize size of the node internal memory
	 * @param inputs indexes of the slots to use as this node inputs
	 * @param outputs indexes of the slots to use as this node outputs
	 * @param operations operations to execute on the node
	 * @return this
	 */
	public JavaTzEnv addImplNode(
		final String name,
		final int memorySize,
		final int[] inputs,
		final int[] outputs,
		final List<Operation> operations) {
		final Node node = new Node(
			memorySize,
			getInputs(this.slots, inputs),
			getOutputs(this.slots, outputs));
		final NodeExecution execution = new NodeExecution(node, operations);

		final NodeExecution previousExecution = this.nodes.put(name, execution);
		if (previousExecution != null) {
			throw new IllegalStateException("Existing node registered under " + name);
		}

		if (logger.isLoggable(Level.FINE)) {
			logger.fine(String.format(
				"New node added to environment: %s[%d].%nInputs are %s%nOutputs are%s",
				name,
				memorySize,
				Arrays.toString(inputs),
				Arrays.toString(outputs)));
		}

		return this;
	}

	/**
	 * Configures a consumer for this environment data.
	 * <p>
	 *   Without configuration, output data is swallowed.
	 * </p>
	 * @param consumer consumer of produced data
	 * @return this
	 */
	public JavaTzEnv produceInto(Consumer<OptionalInt[]> consumer) {
		this.consumer = consumer;
		return this;
	}

	/**
	 * Feeds this environment with data.
	 * @param input input values to feed to the input slots.
	 */
	public void consume(final int[] input) {
		for (int i = 0, end_ = Math.max(input.length, inputs.length); i < end_; i += 1) {
			this.inputs[i].enqueue(input[i]);
		}
	}

	/**
	 * Executes a tick of all nodes in this environment.
	 */
	public void tick() {
		Stream.of(this.slots).forEach(TransactionalElement::onStepStart);

		this.nodes.values().forEach(NodeExecution::runStep);
		// Complete transaction for each element
		Stream.of(this.slots).forEach(TransactionalElement::onStepEnd);

		// Check for an output
		if (Stream.of(this.outputs).anyMatch(DataSlot::canRead)) {
			final OptionalInt[] output = Stream.of(this.outputs)
				.map(o -> o.canRead() ? OptionalInt.of(o.read()) : OptionalInt.empty())
				.toArray(OptionalInt[]::new);
			this.consumer.accept(output);
		}
	}

	/**
	 * Runs this environment using the Java system.
	 * @param args program arguments
	 */
	public void runFromSystem(final String[] args) {
		SystemExecutor.fromSystem().run(this);
	}

	@Override
	public Stream<OptionalInt[]> runOn(final Stream<int[]> inputs, int cycles) {
		final StaticExecutor executor = StaticExecutor.on(inputs, cycles);
		executor.run(this);
		return executor.getResult();
	}

	/**
	 * Extracts a selection of slots into an array.
	 * @param slots all slots
	 * @param indexes indexes of slots to extract
	 * @param generator constructor of the resulting array
	 * @param <T> Implementation type of the selected slots
	 * @return the created selection
	 */
	@SuppressWarnings("unchecked")
	private static <T> T[] getSlots(final Object[] slots, final int[] indexes, final IntFunction<T[]> generator) {
		try {
			return IntStream.of(indexes)
				.mapToObj(i -> (T) slots[i])
				.toArray(generator);
		} catch (ArrayStoreException e) {
			// The collected slots do not match the expected type. Add a nice message for debug
			final StringBuilder message = new StringBuilder("Failed to collect the slots. Selected:");
			final Set<Integer> idx = IntStream.of(indexes).boxed().collect(Collectors.toSet());
			for (int i = 0; i < slots.length; i += 1) {
				message.append("\n -")
					.append(idx.contains(i) ? '>' : ' ')
					.append(' ')
					.append(slots[i]);
			}
			throw new RuntimeException(message.toString(), e);
		}
	}

	/**
	 * Extracts a selection of input slots from a selection of slots.
	 * @param slots all slots
	 * @param inputs indexes of slots to extract
	 * @return selected inputs
	 */
	private static InputSlot[] getInputs(final Object[] slots, final int[] inputs) {
		return getSlots(slots, inputs, InputSlot[]::new);
	}

	/**
	 * Extracts a selection of output slots from a selection of slots.
	 * @param slots all slots
	 * @param outputs indexes of slots to extract
	 * @return selected outputs
	 */
	private static OutputSlot[] getOutputs(final Object[] slots, final int[] outputs) {
		return getSlots(slots, outputs, OutputSlot[]::new);
	}

}
