package com.kineolyan.tzio.v1.java.slot;

/**
 * Representation of an input slot, that can be read if some data are available.
 */
public interface InputSlot {

	/**
	 * Gets if data are available and can be read with {@link #read()}.
	 * @return true if there are data, false otherwise
	 */
	boolean canRead();

	/**
	 * Reads one value in the available data.
	 * <p>
	 *   This assumes that {@link #canRead()} is true.
	 * </p>
	 * @return read value
	 */
	int read();

}
