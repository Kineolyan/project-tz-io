package com.kineolyan.tzio.v1;

import com.kineolyan.tzio.v1.ops.Operation;
import com.kineolyan.tzio.v1.slot.DataSlot;
import com.kineolyan.tzio.v1.slot.InputQueueSlot;
import com.kineolyan.tzio.v1.slot.InputSlot;
import com.kineolyan.tzio.v1.slot.OutputSlot;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import java.util.function.IntFunction;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * Representation of a whole TZ-IO environment.
 * <p>
 *   This contains the nodes in the environment, the operations to run on each node.
 *   It connects nodes between each other, as well as nodes to the outside world.
 * </p>
 */
public class TzEnv {

	/** Output value marking the absence of value for the given output slot */
	public static int NO_OUTPUT = -1;

	/** Map of node executions indexed by node names */
	private Map<String, NodeExecution> nodes;
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
	private Consumer<int[]> consumer;

	/**
	 * Constructor
	 */
	public TzEnv() {
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
	public TzEnv withSlots(
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
		return this;
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
	public TzEnv addNode(
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
	public TzEnv produceInto(Consumer<int[]> consumer) {
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
			final int[] output = Stream.of(this.outputs)
				.mapToInt(o -> o.canRead() ? o.read() : NO_OUTPUT)
				.toArray();
			this.consumer.accept(output);
		}
	}

	/**
	 * Extracts a selection of slots into an array.
	 * @param slots all slots
	 * @param indexes indexes of slots to extract
	 * @param generator constructor of the resulting array
	 * @param <T> Implementation type of the selected slots
	 * @return the created selection
	 */
	private static <T> T[] getSlots(final Object[] slots, final int[] indexes, final IntFunction<T[]> generator) {
		return IntStream.of(indexes)
			.mapToObj(i -> (T) slots[i])
			.toArray(generator);
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
