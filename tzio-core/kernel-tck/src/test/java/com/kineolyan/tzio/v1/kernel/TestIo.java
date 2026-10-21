package com.kineolyan.tzio.v1.kernel;

import static com.kineolyan.tzio.v1.api.ops.Operations.ADD;
import static com.kineolyan.tzio.v1.api.ops.Operations.MOV;
import static com.kineolyan.tzio.v1.api.ref.References.inAcc;
import static com.kineolyan.tzio.v1.api.ref.References.inNil;
import static com.kineolyan.tzio.v1.api.ref.References.inSlot;
import static com.kineolyan.tzio.v1.api.ref.References.outAcc;
import static com.kineolyan.tzio.v1.api.ref.References.outNil;
import static com.kineolyan.tzio.v1.api.ref.References.outSlot;
import static com.kineolyan.tzio.v1.api.ref.References.value;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

@DisplayName("Kernel inputs and outputs")
class TestIo {

	@Test
	@DisplayName("Moving an input to an output consumes and produces in the same tick")
	void testMoveInputToOutput() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.expectSilence()
					.sync("0:42", "-")
					.expect("IO 0 0:42");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Reading an input only reports the consumption")
	void testConsumeOnly() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				ADD(inSlot(1)),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("0:5", "-")
					.expect("IO 0 -", "IO - 0:5");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Writing an immediate value only reports the production")
	void testProduceOnly() {
		try (KernelDriver kernel = KernelDriver.run(0, 1, 0, MOV(value(-7), outSlot(1)))) {
			kernel.expect("IO - 0:-7");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("An occupied output blocks until freed, without consuming the input")
	void testOccupiedOutput() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.sync("0:1", "-")
					.expect("IO 0 0:1")
					.sync("0:2", "-")
					.expectSilence()
					.sync("-", "0")
					.expect("IO 0 0:2");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("A SYNC can carry new values and freed outputs together")
	void testValuesAndFrees() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0, MOV(inSlot(1), outSlot(1)))) {
			kernel.sync("0:1", "-")
					.expect("IO 0 0:1")
					.sync("0:2", "0")
					.expect("IO 0 0:2")
					.sync("0:3", "0")
					.expect("IO 0 0:3");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("A SYNC can carry values for several inputs")
	void testSeveralInputs() {
		try (KernelDriver kernel = KernelDriver.run(3, 1, 0,
				MOV(inSlot(3), outAcc()),
				ADD(inSlot(1)),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("0:10,2:5", "-")
					.expect("IO 2 -", "IO 0 -", "IO - 0:15");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Values received while blocked on another input are kept")
	void testBufferedInput() {
		try (KernelDriver kernel = KernelDriver.run(2, 1, 0,
				MOV(inSlot(1), outAcc()),
				ADD(inSlot(2)),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("1:4", "-")
					.expectSilence()
					.sync("0:3", "-")
					.expect("IO 0 -", "IO 1 -", "IO - 0:7");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Inputs and outputs are addressed up to index 1023")
	void testHighIndexes() {
		try (KernelDriver kernel = KernelDriver.run(1024, 1024, 0, MOV(inSlot(1024), outSlot(1024)))) {
			kernel.sync("1023:9", "-")
					.expect("IO 1023 1023:9")
					.sync("1023:8", "1023")
					.expect("IO 1023 1023:8");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Each output is used independently")
	void testSeveralOutputs() {
		try (KernelDriver kernel = KernelDriver.run(1, 3, 0,
				MOV(inSlot(1), outSlot(3)),
				MOV(inSlot(1), outSlot(1)))) {
			kernel.sync("0:1", "-")
					.expect("IO 0 2:1")
					.sync("0:2", "-")
					.expect("IO 0 0:2")
					// Output 2 is still occupied
					.sync("0:3", "-")
					.expectSilence()
					.sync("-", "2")
					.expect("IO 0 2:3");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Moving to NIL consumes the input without producing")
	void testMoveToNil() {
		try (KernelDriver kernel = KernelDriver.run(1, 0, 0, MOV(inSlot(1), outNil()))) {
			kernel.sync("0:1", "-")
					.expect("IO 0 -")
					.sync("0:2", "-")
					.expect("IO 0 -");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("NIL reads as 0")
	void testMoveFromNil() {
		try (KernelDriver kernel = KernelDriver.run(0, 1, 0,
				MOV(value(5), outAcc()),
				MOV(inNil(), outAcc()),
				MOV(inAcc(), outSlot(1)))) {
			kernel.expect("IO - 0:0");
			kernel.stop();
		}
	}

}
