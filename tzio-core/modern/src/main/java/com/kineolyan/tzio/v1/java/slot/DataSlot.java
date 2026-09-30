package com.kineolyan.tzio.v1.java.slot;

import com.kineolyan.tzio.v1.java.TransactionalElement;

/**
 * Input/Output slot storing a single value.
 */
public class DataSlot implements InputSlot, OutputSlot, TransactionalElement {

	/** Value in the slot */
	private int value = 0;
	/** Flag marking that a value is currently stored */
	private boolean hasValue = false;
	/** Flag marking that the value has been consumed */
	private boolean hasValueAfterStep = false;

	/**
	 * Gets the value of the slot.
	 * @return the value
	 */
	public int getValue() {
		return this.value;
	}

	@Override
	public boolean canRead() {
		return this.hasValue && this.hasValueAfterStep;
	}

	@Override
	public int read() {
		assert canRead(): "Cannot read from this slot";
		this.hasValueAfterStep = false;
		return this.value;
	}

	@Override
	public boolean canWrite() {
		return !this.hasValue && !this.hasValueAfterStep;
	}

	@Override
	public void write(final int value) {
		assert canWrite(): "Cannot write into this slot";
		this.value = value;
		this.hasValueAfterStep = true;
	}

	@Override
	public void onStepEnd() {
		this.hasValue = this.hasValueAfterStep;
	}
}
