package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ref.InputReference;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;

import java.util.function.BiConsumer;

/**
 * Operation doing arithmetic changes on the node value.
 */
@RequiredArgsConstructor(access = AccessLevel.PRIVATE)
final class IncrementOperation implements Operation {

	/** Input containing the incremental value */
	private final InputReference input;
	/** Operation on the node with the input readable value */
	private final BiConsumer<Node, InputReference> operation;

	@Override
	public Shift execute(Node node) {
		if (this.input.canRead(node)) {
			this.operation.accept(node, this.input);
			return Shift.NEXT;
		} else {
			return Shift.STAY;
		}
	}

	/**
	 * Creates an operation adding the input value to the node value.
	 * @param input input containing the value to add
	 * @return the created operation
	 */
	public static IncrementOperation add(final InputReference input) {
		return new IncrementOperation(input, Node::addValue);
	}

	/**
	 * Creates an operation subtracting the input value to the node value.
	 * @param input input containing the value to subtract
	 * @return the created operation
	 */
	public static IncrementOperation sub(final InputReference input) {
		return new IncrementOperation(input, Node::subValue);
	}

}
