package com.kineolyan.tzio.v1.multi;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import com.kineolyan.tzio.v1.api.ops.Operations;
import com.kineolyan.tzio.v1.api.ref.References;
import com.kineolyan.tzio.v1.samples.SampleHelper;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.IntStream;
import org.junit.jupiter.api.Test;

class TestMultiTzEnv {

	private static final String PYTHON_KERNEL = "python3 "
			+ Path.of("../../kernels/python/tzk.py").toAbsolutePath().normalize();

	private static MultiTzEnv createEnv(final String... lines) {
		final KernelConfig config = KernelConfig.parse(List.of(lines), Path.of(".").toAbsolutePath());
		return new MultiTzEnv(() -> config);
	}

	/**
	 * Creates a program computing the absolute values of the input, then swapping them by pairs.
	 */
	private static MultiTzEnv createAbsSwapProgram() {
		return createEnv("abs => py", "swap => py", "py <= " + PYTHON_KERNEL)
				.withSlots(3, new int[]{0}, new int[]{2})
				.addNode("abs", 0, new int[]{0}, new int[]{1}, List.of(
						Operations.MOV(References.inSlot(1), References.outAcc()),
						Operations.JLZ("neg"),
						Operations.JRO(References.value(2)),
						Operations.LABEL("neg"),
						Operations.NEG(),
						Operations.MOV(References.inAcc(), References.outSlot(1))))
				.addNode("swap", 1, new int[]{1}, new int[]{2}, List.of(
						Operations.MOV(References.inSlot(1), References.outAcc()),
						Operations.SAV(1),
						Operations.MOV(References.inSlot(1), References.outAcc()),
						Operations.MOV(References.inAcc(), References.outSlot(1)),
						Operations.SWP(1),
						Operations.MOV(References.inAcc(), References.outSlot(1))));
	}

	@Test
	void testJumpsAndMemory() {
		final List<List<Integer>> output = SampleHelper.batchCollect(createAbsSwapProgram()
				.runOn(new IntStream[]{IntStream.of(3, -4, 0, -5)}, 100));
		assertThat(output).containsExactly(
				List.of(4),
				List.of(3),
				List.of(5),
				List.of(0));
	}

	@Test
	void testTrace() {
		final MultiTzEnv env = createAbsSwapProgram();
		System.setProperty(MultiTzEnv.TRACE_PROPERTY, "true");
		try {
			env.runOn(new IntStream[]{IntStream.of(1, 2)}, 50).toList();
		} finally {
			System.clearProperty(MultiTzEnv.TRACE_PROPERTY);
		}

		assertThat(env.trace()).containsOnlyKeys("abs", "swap");
		assertThat(env.trace().get("abs"))
				.isNotEmpty()
				.allMatch(line -> line.matches("@S \\d+ \\d+ -?\\d+ (RUN|WAIT_IN|WAIT_OUT)"));
		assertThat(env.trace().get("swap"))
				.isNotEmpty()
				.allMatch(line -> line.matches("@S \\d+ \\d+ -?\\d+ (RUN|WAIT_IN|WAIT_OUT) -?\\d+"));
	}

	@Test
	void testNoTraceByDefault() {
		final MultiTzEnv env = createAbsSwapProgram();
		env.runOn(new IntStream[]{IntStream.of(1, 2)}, 10).toList();
		assertThat(env.trace().values()).allMatch(List::isEmpty);
	}

	@Test
	void testDeadKernelFailsTheProgram() {
		final MultiTzEnv env = createEnv("node => dead", "dead <= read line; exit 3")
				.withSlots(2, new int[]{0}, new int[]{1})
				.addNode("node", 0, new int[]{0}, new int[]{1}, List.of(
						Operations.MOV(References.inSlot(1), References.outSlot(1))));
		assertThatThrownBy(() -> env.runOn(new IntStream[]{IntStream.of(1)}, 10))
				.hasMessageContaining("node")
				.hasMessageContaining("exited with code 3");
	}

	@Test
	void testSharedSlotIsRejected() {
		final MultiTzEnv env = createEnv()
				.withSlots(2, new int[]{0}, new int[]{1})
				.addNode("a", 0, new int[]{0}, new int[]{1}, List.of(Operations.NEG()));
		assertThatThrownBy(() -> env.addNode("b", 0, new int[]{0}, new int[]{}, List.of(Operations.NEG())))
				.isInstanceOf(IllegalArgumentException.class)
				.hasMessageContaining("slot 0");
	}

}
