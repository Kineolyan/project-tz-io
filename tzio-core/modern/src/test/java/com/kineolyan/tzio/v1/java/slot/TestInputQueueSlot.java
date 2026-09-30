package com.kineolyan.tzio.v1.java.slot;

import org.junit.jupiter.api.Test;

import java.util.stream.IntStream;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

class TestInputQueueSlot {

	@Test
	void testStartWithoutValues() {
		final InputQueueSlot input = new InputQueueSlot();
		assertThat(input.canRead()).isFalse();
	}

	/**
	 * Tests that {@link InputQueueSlot#canRead()} can be called many times without
	 * impacting the values.
	 */
	@Test
	void testCanReadManyTimes() {
		final InputQueueSlot input = new InputQueueSlot();
		input.enqueue(1);
		IntStream.range(0, 5).forEach(i -> assertThat(input.canRead()).isTrue());
		assertThat(input.read()).isEqualTo(1);
	}

	@Test
	void testReadingValues() {
		final InputQueueSlot input = new InputQueueSlot();
		input.enqueue(2);
		input.enqueue(1);

		assertThat(input.canRead());
		assertThat(input.read()).isEqualTo(2);

		assertThat(input.canRead());
		assertThat(input.read()).isEqualTo(1);

		assertThat(input.canRead()).isFalse();
	}

	@Test
	void testFailureOnReadWithoutValue() {
		final InputQueueSlot input = new InputQueueSlot();
		input.enqueue(1);

		input.read();
		assertThat(input.canRead()).isFalse();
		assertThatThrownBy(() -> input.read())
			.isInstanceOf(IllegalStateException.class)
			.hasMessageContaining("without values");
	}

}
