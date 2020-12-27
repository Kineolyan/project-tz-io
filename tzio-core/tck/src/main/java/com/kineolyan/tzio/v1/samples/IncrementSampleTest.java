package com.kineolyan.tzio.v1.samples;

import java.util.List;
import java.util.stream.IntStream;
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
public class IncrementSampleTest {

	private static TzEnv create() {
		return TzSystem.getInstance().createEnv()
				.withSlots(2, new int[]{0}, new int[]{1})
				.addNode(
						"1",
						1,
						new int[]{0},
						new int[]{1},
						List.of(
								Operations.MOV(References.inSlot(1), References.outAcc()),
								Operations.ADD(References.value(1)),
								Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	@Test
	@DisplayName("Increment program")
	protected void testProgram() {
		final var inputs = new IntStream[] {IntStream.of(0, 12, -43)};
		final List<List<Integer>> output = SampleHelper.batchCollect(
			create().runOn(inputs, 100));
		assertThat(output).containsExactly(
			List.of(1),
			List.of(13),
			List.of(-42));
	}

	public static void main(final String[] args) {
		create().runFromSystem(args);
	}

}
