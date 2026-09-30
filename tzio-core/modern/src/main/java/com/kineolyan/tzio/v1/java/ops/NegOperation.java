package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

/**
 * Operation negating the node internal value.
 */
class NegOperation implements Operation {

	/** Singleton instance of this operation */
	public static final NegOperation INSTANCE = new NegOperation();

	/** Hidden constructor */
	private NegOperation() {}

	@Override
	public Shift execute(final Node node) {
		node.negate();
		return Shift.NEXT;
	}
}
