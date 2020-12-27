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
public class DiffsSampleTest {

	private static TzEnv createNode1(TzEnv env) {
		// Double the input value
		return env.addNode(
				"1",
				1,
				new int[]{0, 1},
				new int[]{2, 3},
				List.of(
						Operations.MOV(References.inSlot(1), References.outAcc()),
						Operations.SUB(References.inSlot(2)),
						Operations.MOV(References.inAcc(), References.outSlot(1)),
						Operations.NEG(),
						Operations.MOV(References.inAcc(), References.outSlot(2))));
	}

	private static TzEnv create() {
		return createNode1(
				TzSystem.getInstance().createEnv()
						.withSlots(4, new int[]{0, 1}, new int[]{2, 3}));
	}

	@Test
	@DisplayName("Double program")
	protected void testProgram() {
		final var inputs = new IntStream[]{
				IntStream.of(1, 3),
				IntStream.of(2, -4)
		};
		final List<List<Integer>> output = SampleHelper.batchCollect(
				create().runOn(inputs, 99));
		assertThat(output).containsExactly(
				List.of(-1, 1),
				List.of(7, -7));
	}

	public static void main(final String[] args) {
		create().runFromSystem(args);
	}

}
