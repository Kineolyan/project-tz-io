package com.kineolyan.tzio.v1.java.slot;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestDataSlot {

	@Test
	void testCanRead() {
		final DataSlot slot = new DataSlot();

		assertThat(slot.canRead()).isFalse();
		write(slot, 1);

		assertThat(slot.canRead()).isTrue();
	}

	@Test
	void testRead() {
		final DataSlot slot = new DataSlot();
		write(slot, 2);

		assertThat(slot.canRead()).isTrue();
		assertThat(slot.read()).isEqualTo(2);
		assertThat(slot.canRead()).isFalse();
	}

	@Test
	void testCanWrite() {
		final DataSlot slot = new DataSlot();
		assertThat(slot.canWrite()).isTrue();

		slot.write(3);
		assertThat(slot.canWrite()).isFalse();
	}

	@Test
	void testWrite() {
		final DataSlot slot = new DataSlot();
		write(slot, 4);
		assertThat(slot.read()).isEqualTo(4);
	}

	@Test
	void testCannotReadBeforeTransactionEnd() {
		final DataSlot slot = new DataSlot();
		slot.write(5);
		assertThat(slot.canWrite()).isFalse();
		assertThat(slot.canRead()).isFalse();

		slot.onStepEnd();
		assertThat(slot.canRead()).isTrue();
		assertThat(slot.canWrite()).isFalse();
	}

	@Test
	void testCannotWriteBeforeTransactionEnd() {
		final DataSlot slot = new DataSlot();
		write(slot, 6);

		slot.read();
		assertThat(slot.canRead()).isFalse();
		assertThat(slot.canWrite()).isFalse();

		slot.onStepEnd();
		assertThat(slot.canWrite()).isTrue();
		assertThat(slot.canRead()).isFalse();
	}

	private static void write(final DataSlot slot, final int value) {
		slot.write(value);
		slot.onStepEnd();
	}

}
