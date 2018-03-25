package com.kineolyan.tzio.ref;

import com.kineolyan.tzio.Node;

/**
 * Representation of a reference to an output.
 * <p>
 *   The reference can check that the output can be written and can perform the write.
 * </p>
 */
public interface OutputReference {

	/**
	 * Tests that the referenced output can be written.
	 * @param node node targeted by the reference
	 * @return true if the output is writable
	 */
	boolean canWrite(Node node);

	/**
	 * Writes the given value into the node output.
	 * @param node node to consider
	 * @param value value to write
	 */
	void writeValue(Node node, int value);

}
