package com.kineolyan.tzio.v1.java.slot;

/**
 * Representation of a node output slot, where values can be written.
 */
public interface OutputSlot {

	/**
	 * Gets if it is possible to write one new value in the output.
	 * @return true if the output is writable, false otherwise
	 */
	boolean canWrite();

	/**
	 * Writes a value into the output.
	 * <p>
	 *   This assumes that {@link #canWrite()} is true.
	 * </p>
	 * @param value value to write
	 */
	void write(int value);

}
