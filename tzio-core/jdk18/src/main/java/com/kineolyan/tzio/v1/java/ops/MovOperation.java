package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ref.InputReference;
import com.kineolyan.tzio.v1.java.ref.OutputReference;

/**
 * Operation assigning the value of an input into an output.
 */
class MovOperation implements Operation {

	/** Input to read for a value */
	private final InputReference from;
	/** Output where the value is written */
	private final OutputReference to;

	/**
	 * Constructor
	 * @param from input to read
	 * @param to output to write
	 */
	public MovOperation(final InputReference from, final OutputReference to) {
		this.from = from;
		this.to = to;
	}

	@Override
	public Shift execute(final Node node) {
		if (this.from.canRead(node) && this.to.canWrite(node)) {
			node.moveValue(this.from, this.to);
			return Operation.Shift.NEXT;
		} else {
			return Operation.Shift.STAY;
		}
	}
}
