package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

/**
 * Empty operation, serving as a label point for "goto" operators.
 *
 * @param label Label of the operation
 */
record LabelOperation(String label) implements Operation {

	/**
	 * Constructor
	 *
	 * @param label operation label
	 */
	LabelOperation {
	}


	@Override
	public Shift execute(final Node node) {
		throw new UnsupportedOperationException("Cannot execute a label operation");
	}
}
