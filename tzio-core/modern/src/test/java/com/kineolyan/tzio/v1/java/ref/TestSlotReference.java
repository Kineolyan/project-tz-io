package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.slot.DataSlot;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * @author ActiveViam
 */
class TestSlotReference {

	private Node node;
	private DataSlot inputSlot;
	private DataSlot outputSlot;

	@BeforeEach
	void prepareEnv() {
		this.inputSlot = new DataSlot();
		this.outputSlot = new DataSlot();
		this.node = new Node(
				1,
				new InputSlot[]{this.inputSlot},
				new OutputSlot[]{this.outputSlot});
	}

	@Test
	void testCanReadFromInput() {
		final SlotReference ref = SlotReference.of(1);
		assertThat(ref.canRead(this.node)).isFalse();

		writeSlot(this.inputSlot, 1);

		assertThat(ref.canRead(this.node)).isTrue();
	}

	@Test
	void testReadFromInput() {
		writeSlot(this.inputSlot, 9);
		writeSlot(this.outputSlot, 4);

		assertThat(SlotReference.of(1).readValue(this.node)).isEqualTo(9);
	}

	@Test
	void testCanWriteIntoOutput() {
		final SlotReference ref = SlotReference.of(1);
		writeSlot(this.outputSlot, 4);
		assertThat(ref.canWrite(this.node)).isFalse();

		readSlot(this.outputSlot);
		assertThat(ref.canWrite(this.node)).isTrue();
	}

	@Test
	void testWriteIntoOutput() {;
		SlotReference.of(1).writeValue(this.node, 46);
		this.outputSlot.onStepEnd();

		assertThat(readSlot(this.outputSlot)).isEqualTo(46);
	}

	private int readSlot(final DataSlot slot) {
		final int result = slot.read();
		slot.onStepEnd();
		return result;
	}

	private void writeSlot(final DataSlot slot, final int value) {
		slot.write(value);
		slot.onStepEnd();
	}

}
