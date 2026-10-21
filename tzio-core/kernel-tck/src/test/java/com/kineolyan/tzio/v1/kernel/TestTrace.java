package com.kineolyan.tzio.v1.kernel;

import static com.kineolyan.tzio.v1.api.ops.Operations.MOV;
import static com.kineolyan.tzio.v1.api.ops.Operations.SAV;
import static com.kineolyan.tzio.v1.api.ref.References.inAcc;
import static com.kineolyan.tzio.v1.api.ref.References.inSlot;
import static com.kineolyan.tzio.v1.api.ref.References.outAcc;
import static com.kineolyan.tzio.v1.api.ref.References.outSlot;
import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

@DisplayName("Kernel state trace")
class TestTrace {

	private static List<String> stateLines(final KernelDriver kernel) {
		return kernel.stderr().stream()
				.filter(line -> line.startsWith("@S "))
				.toList();
	}

	@Test
	@DisplayName("State lines follow the format '@S <tick> <pc> <acc> <status> <mem...>'")
	void testFormat() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 2,
				MOV(inSlot(1), outAcc()),
				SAV(2),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("0:7", "-").expect("IO 0 -", "IO - 0:7");
			kernel.stop();

			final List<String> states = stateLines(kernel);
			assertThat(states)
					.isNotEmpty()
					.allMatch(line -> line.matches("@S \\d+ \\d+ -?\\d+ (RUN|WAIT_IN|WAIT_OUT) -?\\d+ -?\\d+"));
			// After SAV 2, the second memory slot holds the accumulator
			assertThat(states).anyMatch(line -> line.matches("@S \\d+ 2 7 RUN 0 7"));
		}
	}

	@Test
	@DisplayName("A node waiting for an input reports WAIT_IN on its operation")
	void testWaitIn() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.expectSilence();
			kernel.stop();

			assertThat(stateLines(kernel)).first().asString().matches("@S \\d+ 0 0 WAIT_IN");
		}
	}

	@Test
	@DisplayName("A node waiting for an output reports WAIT_OUT on its operation")
	void testWaitOut() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.sync("0:1", "-")
					.expect("IO 0 0:1")
					.sync("0:2", "-")
					.expectSilence();
			kernel.stop();

			assertThat(stateLines(kernel)).last().asString().matches("@S \\d+ 0 0 WAIT_OUT");
		}
	}

}
