package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;

import java.util.function.IntPredicate;

/**
 * Representation of a reference to an input.
 * <p>
 *   The reference can check that the input can provide a value and can read it.
 * </p>
 */
public interface InputReference {

	/**
	 * Tests that the referenced input can be read from the node.
	 * @param node node to consider
	 * @return true if the input is available, false otherwise
	 */
	boolean canRead(Node node);

	/**
	 * Reads an input from a node.
	 * <p>
	 *   This assumes that {@link #canRead(Node)} returns true.
	 * </p>
	 * @param node node to consider
	 * @return the value read from one node input
	 */
	int readValue(Node node);

	default boolean execute(Node node, IntPredicate consumer) {
		if (canRead(node)) {
			return consumer.test(readValue(node));
		} else {
			return false;
		}
	}

}
