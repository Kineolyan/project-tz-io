package com.kineolyan.tzio.v1.kernel;

import static com.kineolyan.tzio.v1.api.ops.Operations.ADD;
import static com.kineolyan.tzio.v1.api.ops.Operations.JEZ;
import static com.kineolyan.tzio.v1.api.ops.Operations.JGZ;
import static com.kineolyan.tzio.v1.api.ops.Operations.JLZ;
import static com.kineolyan.tzio.v1.api.ops.Operations.JMP;
import static com.kineolyan.tzio.v1.api.ops.Operations.JNZ;
import static com.kineolyan.tzio.v1.api.ops.Operations.JRO;
import static com.kineolyan.tzio.v1.api.ops.Operations.LABEL;
import static com.kineolyan.tzio.v1.api.ops.Operations.MOV;
import static com.kineolyan.tzio.v1.api.ops.Operations.NEG;
import static com.kineolyan.tzio.v1.api.ops.Operations.SAV;
import static com.kineolyan.tzio.v1.api.ops.Operations.SUB;
import static com.kineolyan.tzio.v1.api.ops.Operations.SWP;
import static com.kineolyan.tzio.v1.api.ref.References.inAcc;
import static com.kineolyan.tzio.v1.api.ref.References.inNil;
import static com.kineolyan.tzio.v1.api.ref.References.inSlot;
import static com.kineolyan.tzio.v1.api.ref.References.outAcc;
import static com.kineolyan.tzio.v1.api.ref.References.outSlot;
import static com.kineolyan.tzio.v1.api.ref.References.value;

import com.kineolyan.tzio.v1.api.ops.OperationType;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

/**
 * Tests of the instructions, observed through the values the node writes on its outputs.
 */
@DisplayName("Kernel instructions")
class TestInstructions {

	/**
	 * Feeds a value to input 0, expecting it to be consumed, then a value on output 0.
	 * <p>
	 *   Output 0 is freed afterwards.
	 * </p>
	 */
	private static void assertOutput(final KernelDriver kernel, final int input, final int output) {
		kernel.sync("0:" + input, "-")
				.expect("IO 0 -", "IO - 0:" + output)
				.sync("-", "0");
	}

	@Test
	@DisplayName("ADD and SUB with slots, ACC, NIL and immediate values")
	void testAddSub() {
		try (KernelDriver kernel = KernelDriver.run(2, 1, 0,
				MOV(inSlot(1), outAcc()),
				ADD(inAcc()),
				SUB(inSlot(2)),
				ADD(value(-5)),
				SUB(value(2)),
				ADD(inNil()),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("0:10,1:4", "-")
					// 10 + 10 - 4 - 5 - 2 + 0
					.expect("IO 0 -", "IO 1 -", "IO - 0:9");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("SUB ACC resets the accumulator")
	void testSubAcc() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				SUB(inAcc()),
				MOV(inAcc(), outSlot(1)))) {
			assertOutput(kernel, 7, 0);
			kernel.stop();
		}
	}

	@ParameterizedTest(name = "{0} + 1 = {1}")
	@CsvSource({
			"2147483647, -2147483648",
			"-1, 0"
	})
	@DisplayName("ADD wraps as a 32-bit integer")
	void testAddOverflow(final int input, final int output) {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				ADD(value(1)),
				MOV(inAcc(), outSlot(1)))) {
			assertOutput(kernel, input, output);
			kernel.stop();
		}
	}

	@Test
	@DisplayName("SUB wraps as a 32-bit integer")
	void testSubOverflow() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				SUB(value(1)),
				MOV(inAcc(), outSlot(1)))) {
			assertOutput(kernel, Integer.MIN_VALUE, Integer.MAX_VALUE);
			kernel.stop();
		}
	}

	@Test
	@DisplayName("NEG negates the accumulator")
	void testNeg() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				NEG(),
				MOV(inAcc(), outSlot(1)))) {
			assertOutput(kernel, 5, -5);
			assertOutput(kernel, -3, 3);
			assertOutput(kernel, 0, 0);
			kernel.stop();
		}
	}

	@Test
	@DisplayName("SAV and SWP use distinct memory slots")
	void testMemory() {
		try (KernelDriver kernel = KernelDriver.run(1, 2, 3,
				MOV(inSlot(1), outAcc()),
				SAV(3),
				MOV(inSlot(1), outAcc()),
				SAV(1),
				MOV(inSlot(1), outAcc()),
				SWP(3),
				MOV(inAcc(), outSlot(1)),
				SWP(1),
				MOV(inAcc(), outSlot(2)),
				SWP(3),
				MOV(inAcc(), outSlot(1)))) {
			kernel.sync("0:1", "-").expect("IO 0 -")
					.sync("0:2", "-").expect("IO 0 -")
					.sync("0:3", "-")
					// mem=[2, 0, 1], acc=3 -> SWP 3: acc=1 -> SWP 1: acc=2 -> SWP 3: acc=3
					.expect("IO 0 -", "IO - 0:1", "IO - 1:2")
					.sync("-", "0")
					.expect("IO - 0:3");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("Execution loops to the first operation after the last one")
	void testLoop() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				ADD(value(100)),
				MOV(inAcc(), outSlot(1)))) {
			assertOutput(kernel, 1, 101);
			assertOutput(kernel, 2, 102);
			kernel.stop();
		}
	}

	@Test
	@DisplayName("JMP always jumps")
	void testJmp() {
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				LABEL("start"),
				MOV(inSlot(1), outAcc()),
				JMP("out"),
				NEG(),
				LABEL("out"),
				MOV(inAcc(), outSlot(1)),
				JMP("start"),
				MOV(value(99), outSlot(1)))) {
			assertOutput(kernel, 4, 4);
			assertOutput(kernel, 5, 5);
			kernel.stop();
		}
	}

	@ParameterizedTest(name = "{0} with ACC={1} jumps: {2}")
	@CsvSource({
			"JEZ, -1, false", "JEZ, 0, true", "JEZ, 1, false",
			"JNZ, -1, true", "JNZ, 0, false", "JNZ, 1, true",
			"JLZ, -1, true", "JLZ, 0, false", "JLZ, 1, false",
			"JGZ, -1, false", "JGZ, 0, false", "JGZ, 1, true"
	})
	@DisplayName("Conditional jumps test the accumulator")
	void testConditionalJumps(final String jump, final int acc, final boolean taken) {
		final OperationType condition = switch (jump) {
			case "JEZ" -> JEZ("taken");
			case "JNZ" -> JNZ("taken");
			case "JLZ" -> JLZ("taken");
			case "JGZ" -> JGZ("taken");
			default -> throw new IllegalArgumentException(jump);
		};
		try (KernelDriver kernel = KernelDriver.run(1, 1, 0,
				MOV(inSlot(1), outAcc()),
				condition,
				MOV(value(0), outSlot(1)),
				JMP("end"),
				LABEL("taken"),
				MOV(value(1), outSlot(1)),
				LABEL("end"))) {
			assertOutput(kernel, acc, taken ? 1 : 0);
			kernel.stop();
		}
	}

	@ParameterizedTest(name = "JRO {0} -> {1}")
	@CsvSource({
			"1, 10",
			"3, 20",
			"5, 30",
			// Offsets wrap around the 7 operations
			"-2, 30",
			"8, 10",
			"-6, 10"
	})
	@DisplayName("JRO jumps relatively to the current operation")
	void testJro(final int offset, final int output) {
		try (KernelDriver kernel = createJroProgram()) {
			assertOutput(kernel, offset, output);
			kernel.stop();
		}
	}

	@Test
	@DisplayName("JRO 0 stays on the same operation")
	void testJroZero() {
		try (KernelDriver kernel = createJroProgram()) {
			kernel.sync("0:0", "-")
					.expect("IO 0 -")
					// Still on JRO, reading the next offset
					.sync("0:3", "-")
					.expect("IO 0 -", "IO - 0:20");
			kernel.stop();
		}
	}

	@Test
	@DisplayName("JRO accepts an immediate value")
	void testJroImmediate() {
		try (KernelDriver kernel = KernelDriver.run(0, 1, 0,
				JRO(value(2)),
				MOV(value(1), outSlot(1)),
				MOV(value(2), outSlot(1)))) {
			kernel.expect("IO - 0:2")
					.sync("-", "0")
					.expect("IO - 0:2");
			kernel.stop();
		}
	}

	private static KernelDriver createJroProgram() {
		return KernelDriver.run(1, 1, 0,
				LABEL("start"),
				JRO(inSlot(1)),
				MOV(value(10), outSlot(1)),
				JMP("start"),
				MOV(value(20), outSlot(1)),
				JMP("start"),
				MOV(value(30), outSlot(1)),
				JMP("start"));
	}

}
