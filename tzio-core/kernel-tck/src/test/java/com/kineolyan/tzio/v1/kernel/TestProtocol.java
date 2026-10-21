package com.kineolyan.tzio.v1.kernel;

import static com.kineolyan.tzio.v1.api.ops.Operations.MOV;
import static com.kineolyan.tzio.v1.api.ref.References.inSlot;
import static com.kineolyan.tzio.v1.api.ref.References.outSlot;
import static com.kineolyan.tzio.v1.api.ref.References.value;
import static org.assertj.core.api.Assertions.assertThat;

import com.kineolyan.tzio.v1.multi.BytecodeEncoder;
import java.util.Base64;
import java.util.List;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

@DisplayName("Kernel control protocol")
class TestProtocol {

	@Test
	@DisplayName("LOAD is answered with READY, and nothing runs before START")
	void testNothingRunsBeforeStart() {
		try (KernelDriver kernel = KernelDriver.launch()) {
			kernel.load(BytecodeEncoder.encode(0, 1, 0, List.of(MOV(value(1), outSlot(1)))))
					.expectSilence()
					.start()
					.expect("IO - 0:1");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("STOP before START exits the kernel")
	void testStopBeforeStart() {
		try (KernelDriver kernel = KernelDriver.launch()) {
			kernel.load(BytecodeEncoder.encode(1, 1, 0, List.of(MOV(inSlot(1), outSlot(1)))));
			kernel.stop();
		}
	}

	@Test
	@DisplayName("STOP exits a node blocked on an input")
	void testStopWhileBlocked() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.expectSilence();
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Closing stdin exits the kernel")
	void testEndOfInput() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.closeInput();
			assertThat(kernel.awaitExit()).isZero();
		}
	}

	@Test
	@DisplayName("A program without operations does nothing")
	void testEmptyProgram() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0)) {
			kernel.sync("0:1", "-").expectSilence();
			kernel.stop();
		}
	}

	@Test
	@DisplayName("An unsupported bytecode version fails the kernel")
	void testUnsupportedVersion() {
		final byte[] bytecode = BytecodeEncoder.encode(0, 0, 0, List.of());
		bytecode[0] = 2;
		try (KernelDriver kernel = KernelDriver.launch()) {
			kernel.send("LOAD " + Base64.getEncoder().encodeToString(bytecode));
			assertThat(kernel.awaitExit()).isNotZero();
		}
	}

}
