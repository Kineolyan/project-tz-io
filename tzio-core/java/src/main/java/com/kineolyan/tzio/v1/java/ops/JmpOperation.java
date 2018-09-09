package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

import java.util.function.ToIntFunction;

/**
 * Operation shifting the stack to another operation.
 */
class JmpOperation implements Operation, Operation.Shift {

	/** Target label to go when executing this operation */
	private final String targetLabel;

	/**
	 * Constructor
	 * @param targetLabel target label for the operation
	 */
	public JmpOperation(final String targetLabel) {
		this.targetLabel = targetLabel;
	}

	@Override
	public Shift execute(final Node node) {
		return this;
	}

	@Override
	public int update(final ToIntFunction<String> labelIndex, final int current, final int max) {
		return labelIndex.applyAsInt(this.targetLabel);
	}
}
