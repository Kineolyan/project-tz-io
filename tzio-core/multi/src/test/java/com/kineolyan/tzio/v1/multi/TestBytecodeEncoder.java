package com.kineolyan.tzio.v1.multi;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import com.kineolyan.tzio.v1.api.ops.Operations;
import com.kineolyan.tzio.v1.api.ref.References;
import java.util.List;
import org.junit.jupiter.api.Test;

class TestBytecodeEncoder {

	private static byte[] bytes(final int... values) {
		final byte[] result = new byte[values.length];
		for (int i = 0; i < values.length; i += 1) {
			result[i] = (byte) values[i];
		}
		return result;
	}

	@Test
	void testHeader() {
		final byte[] bytecode = BytecodeEncoder.encode(2, 3, 4, List.of(Operations.NEG()));
		assertThat(bytecode).containsExactly(bytes(
				1,
				0, 2,
				0, 3,
				0, 4,
				0, 1,
				0x04));
	}

	@Test
	void testReferences() {
		final byte[] bytecode = BytecodeEncoder.encode(2, 2, 0, List.of(
				Operations.MOV(References.inSlot(1), References.outSlot(2)),
				Operations.MOV(References.inAcc(), References.outNil()),
				Operations.MOV(References.inNil(), References.outAcc())));
		assertThat(bytecode).endsWith(bytes(
				0x01, 0x00, 0x00, 0x00, 0x01,
				0x01, 0xFF, 0xFF, 0x80, 0x00,
				0x01, 0x80, 0x00, 0xFF, 0xFF));
	}

	@Test
	void testImmediateValues() {
		final byte[] bytecode = BytecodeEncoder.encode(0, 1, 0, List.of(
				Operations.MOV(References.value(-2), References.outSlot(1)),
				Operations.ADD(References.value(300)),
				Operations.SUB(References.inAcc()),
				Operations.JRO(References.value(-1))));
		assertThat(bytecode).endsWith(bytes(
				0x81, 0xFF, 0xFF, 0xFF, 0xFE, 0x00, 0x00,
				0x82, 0x00, 0x00, 0x01, 0x2C,
				0x03, 0xFF, 0xFF,
				0x96, 0xFF, 0xFF, 0xFF, 0xFF));
	}

	@Test
	void testMemoryOperations() {
		final byte[] bytecode = BytecodeEncoder.encode(0, 0, 2, List.of(
				Operations.SAV(1),
				Operations.SWP(2)));
		assertThat(bytecode).endsWith(bytes(
				0x05, 0x00, 0x00,
				0x06, 0x00, 0x01));
	}

	@Test
	void testLabelsAreResolved() {
		final byte[] bytecode = BytecodeEncoder.encode(0, 0, 0, List.of(
				Operations.LABEL("start"),
				Operations.NEG(),
				Operations.LABEL("middle"),
				Operations.JEZ("end"),
				Operations.JNZ("middle"),
				Operations.JLZ("start"),
				Operations.JGZ("middle"),
				Operations.JMP("start"),
				Operations.LABEL("end")));
		assertThat(bytecode).containsExactly(bytes(
				1, 0, 0, 0, 0, 0, 0, 0, 6,
				0x04,
				// A label at the end loops to the first operation
				0x12, 0x00, 0x00,
				0x13, 0x00, 0x01,
				0x14, 0x00, 0x00,
				0x15, 0x00, 0x01,
				0x11, 0x00, 0x00));
	}

	@Test
	void testUnknownLabel() {
		assertThatThrownBy(() -> BytecodeEncoder.encode(0, 0, 0, List.of(Operations.JMP("nowhere"))))
				.isInstanceOf(IllegalArgumentException.class)
				.hasMessageContaining("nowhere");
	}

	@Test
	void testSlotOutOfRange() {
		assertThatThrownBy(() -> BytecodeEncoder.encode(0, 0, 0, List.of(
				Operations.ADD(References.inSlot(1025)))))
				.isInstanceOf(IllegalArgumentException.class);
	}

}
