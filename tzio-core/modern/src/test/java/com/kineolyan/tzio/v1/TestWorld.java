package com.kineolyan.tzio.v1;

import com.kineolyan.tzio.v1.java.JavaTzEnv;
import com.kineolyan.tzio.v1.java.ops.Operations;
import com.kineolyan.tzio.v1.java.ref.References;
import java.util.stream.IntStream;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.OptionalInt;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.assertThat;

public class TestWorld {

	@Test
	public void test() {
		final JavaTzEnv env = new JavaTzEnv()
				.withSlots(3, new int[]{0}, new int[]{2})
				.addImplNode(
						"a",
						1,
						new int[]{0},
						new int[]{1},
						List.of(
								Operations.LABEL("start"),
								Operations.MOV(References.inSlot(1), References.outSlot(1))
						))
				.addImplNode(
						"b",
						1,
						new int[]{1},
						new int[]{2},
						List.of(
								Operations.MOV(References.inSlot(1), References.outSlot(1))
						));

		final var inputs = new IntStream[] {IntStream.of(1, 2)};
		final List<List<Integer>> outputs = env.runOn(inputs, 100)
			.map(values -> Stream.of(values)
				.map(OptionalInt::getAsInt)
				.collect(Collectors.toList()))
			.collect(Collectors.toList());
		assertThat(outputs).containsExactly(List.of(1), List.of(2));
	}

}
