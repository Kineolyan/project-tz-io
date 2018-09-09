package com.kineolyan.tzio.v1.java.slot;

import com.kineolyan.tzio.v1.java.TransactionalElement;

import java.util.Queue;
import java.util.concurrent.LinkedBlockingDeque;

/**
 * Input slot storing many values.
 * <p>
 *   This slot mainly represents an system input, where data can come in but not be
 *   immediately consumed by nodes.
 * </p>
 */
public class InputQueueSlot implements InputSlot, TransactionalElement {

	/** Stack of available values */
	private Queue<Integer> values = new LinkedBlockingDeque<>();

	/**
	 * Adds a new value to the input queue.
	 * @param value value to add
	 */
	public void enqueue(final int value) {
		this.values.offer(value);
	}

	@Override
	public boolean canRead() {
		return !this.values.isEmpty();
	}

	@Override
	public int read() {
		final Integer value = this.values.poll();
		if (value != null) {
			return value;
		} else {
			throw new IllegalStateException("Cannot call #read without values");
		}
	}

}
