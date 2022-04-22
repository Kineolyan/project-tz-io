package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

/**
 * Empty operation, serving as a label point for "goto" operators.
 */
class LabelOperation implements Operation {

	/** Label of the operation */
	private final String label;

	/**
	 * Constructor
	 * @param label operation label
	 */
	public LabelOperation(final String label) {
		this.label = label;
	}

	@Override
	public String label() {
		return this.label;
	}

	@Override
	public Shift execute(final Node node) {
		throw new UnsupportedOperationException("Cannot execute a label operation");
	}
}
