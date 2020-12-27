package com.kineolyan.tzio.v1.api;

import java.util.Arrays;
import java.util.List;
import java.util.OptionalInt;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
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
	 * Runs this environment using the Java system.
	 * @param args program arguments
	 */
	void runFromSystem(final String[] args);

	/**
	 * Runs this environment for the provided inputs.
	 * @param inputs
	 * @param cycles
	 * @return
	 */
	Stream<OptionalInt[]> runOn(IntStream[] inputs, int cycles);

	/**
	 * Tests that the environment correctly produces a series of values from a given set
	 * of imput values.
	 * @param inputs input series, indexed by the input position (1-based)
	 * @param outputs expected output series, indexed by the output position (1-based)
	 * @param cycles maximal number of cycles to run produce the output
	 */
	default void testOn(
			final int[][] inputs,
			final int[][] outputs,
			final int cycles) {
		final var inputStreams = Stream.of(inputs)
				.map(IntStream::of)
				.toArray(IntStream[]::new);
		final var outputLists = runOn(inputStreams, cycles)
				.map(values -> Stream.of(values)
						.map(OptionalInt::getAsInt)
						.collect(Collectors.toList()))
				.collect(Collectors.toList());
		System.out.println("Expecting " + Arrays.deepToString(outputs) + " and got " + outputLists);
	}

}
