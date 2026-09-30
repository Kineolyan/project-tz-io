package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

import java.util.function.ToIntFunction;

/**
 * Description of an operation on a {@link Node}.
 */
public interface Operation {

	/**
	 * Gets the label of the operation.
	 * @return operation label.
	 */
	default String label() {
		return null;
	}

	/**
	 * Executes the operation.
	 * @param node node on which the operation is applied
	 * @return the shift on the
	 */
	Shift execute(Node node);

	/**
	 * Representation of a shift to the next operation.
	 */
	interface Shift {

		/** Default shift to the next operation */
		Operation.Shift NEXT = (exec, current, max) -> (current + 1) % max;
		/** Default shift staying at the same operation */
		Operation.Shift STAY = (exec, current, max) -> current;

		/**
		 * Gets the index of the next operation to execute.
		 * @param labelIndex index returning a
		 * @param current current operation index
		 * @param max max operation index - excluded
		 * @return index of the next operation
		 */
		int update(ToIntFunction<String> labelIndex, int current, int max);

	}

}
