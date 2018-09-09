package com.kineolyan.tzio.v1.api;

import java.util.List;
import java.util.OptionalInt;
import java.util.function.Consumer;
import java.util.stream.Stream;

import com.kineolyan.tzio.v1.api.ops.OperationType;

/**
 * Representation of a whole TZ-IO environment.
 * <p>
 *   This contains the nodes in the environment, the operations to run on each node.
 *   It connects nodes between each other, as well as nodes to the outside world.
 * </p>
 */
public interface TzEnv {

	/**
	 * Configure the slots existing in this environment.
	 * @param slotCount total count of slots
	 * @param inputs indexes of slots to use for external inputs
	 * @param outputs indexes of slots to read to produce data
	 * @return this
	 */
	TzEnv withSlots(
			final int slotCount,
			final int[] inputs,
			final int[] outputs);

	/**
	 * Adds a node in this environment.
	 * @param name name of the node
	 * @param memorySize size of the node internal memory
	 * @param inputs indexes of the slots to use as this node inputs
	 * @param outputs indexes of the slots to use as this node outputs
	 * @param operations operations to execute on the node
	 * @return this
	 */
	TzEnv addNode(
			final String name,
			final int memorySize,
			final int[] inputs,
			final int[] outputs,
			final List<OperationType> operations);

	/**
	 * Configures a consumer for this environment data.
	 * <p>
	 *   Without configuration, output data is swallowed.
	 * </p>
	 * @param consumer consumer of produced data
	 * @return this
	 */
	TzEnv produceInto(Consumer<OptionalInt[]> consumer);

	/**
	 * Feeds this environment with data.
	 * @param input input values to feed to the input slots.
	 */
	void consume(final int[] input);

	/**
	 * Runs this environment using the Java system.
	 * @param args program arguments
	 */
	void runFromSystem(final String[] args);

	Stream<OptionalInt[]> runOn(Stream<int[]> inputs, int cycles);

}
