package com.kineolyan.tzio.v1.samples;

import static org.assertj.core.api.Assertions.assertThat;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.arch.TzSystem;
import com.kineolyan.tzio.v1.api.ops.Operations;
import com.kineolyan.tzio.v1.api.ref.References;
import java.util.List;
import java.util.stream.IntStream;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Tests class implementing an node increment the input by one.
 */
public class DoubleSampleTest {

	private static TzEnv createNode1(TzEnv env) {
		// Double the input value
		return env.addNode(
				"1",
				1,
				new int[]{0},
				new int[]{1},
				List.of(
						Operations.MOV(References.inSlot(1), References.outAcc()),
						Operations.ADD(References.inAcc()),
						Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	private static TzEnv create() {
		return createNode1(
				TzSystem.getInstance().createEnv()
						.withSlots(2, new int[]{0}, new int[]{1}));
	}

	@Test
	@DisplayName("Double program")
	protected void testProgram() {
		final var inputs = new IntStream[] {IntStream.of(1, 3, -4)};
		final List<List<Integer>> output = SampleHelper.batchCollect(
				create().runOn(inputs, 100));
		assertThat(output).containsExactly(
				List.of(2),
				List.of(6),
				List.of(-8));
	}

	public static void main(final String[] args) {
		create().runFromSystem(args);
	}

}
