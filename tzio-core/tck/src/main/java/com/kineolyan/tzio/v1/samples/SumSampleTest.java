package com.kineolyan.tzio.v1.samples;

import java.util.List;
import java.util.stream.Stream;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.arch.TzSystem;
import com.kineolyan.tzio.v1.api.ops.Operations;
import com.kineolyan.tzio.v1.api.ref.References;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Tests class implementing an node increment the input by one.
 */
public class SumSampleTest {

	private static TzEnv createNode1(TzEnv env) {
		// Double the input value
		return env.addNode(
			"1",
			1,
			new int[]{0},
			new int[]{2},
			List.of(
				Operations.MOV(References.inSlot(1), References.outAcc()),
				Operations.ADD(References.inAcc()),
				Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	private static TzEnv createNode2(TzEnv env) {
		// Increment in the input
		return env.addNode(
				"2",
				1,
				new int[]{1},
				new int[]{3},
				List.of(
					Operations.MOV(References.inSlot(1), References.outAcc()),
					Operations.ADD(References.value(1)),
					Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	private static TzEnv createNode3(TzEnv env) {
		// Sum the two values
		return env.addNode(
			"3",
			1,
			new int[]{2, 3},
			new int[]{4},
			List.of(
				Operations.MOV(References.inSlot(1), References.outAcc()),
				Operations.ADD(References.inSlot(2)),
				Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	private static TzEnv create() {
		return createNode3(
			createNode2(
				createNode1(
					TzSystem.getInstance().createEnv()
						.withSlots(5, new int[]{0, 1}, new int[]{4}))));
	}

	@Test
	@DisplayName("Sum program")
	protected void testProgram() {
		final Stream<int[]> input = Stream.of(
			new int[] {0, 1},
			new int[] {10, 5},
			new int[] {-43, 86});
		final List<List<Integer>> output = SampleHelper.collect(
			create().runOn(input, 100));
		assertThat(output).containsExactly(
			List.of(2),
			List.of(26),
			List.of(1));
	}

	public static void main(final String[] args) {
		create().runFromSystem(args);
	}

}
