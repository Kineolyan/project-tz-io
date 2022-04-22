package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

import java.util.function.IntPredicate;
import java.util.function.ToIntFunction;

/**
 * Conditional operation jumping to a given label when the consider node value is 0.
 */
class ConditionalOperation implements Operation, Operation.Shift {

	/** Target label operation */
	private final String targetLabel;

	/** Predicate on the node value */
	private final IntPredicate valuePredicate;

	/** Name of the operation for sweet debug */
	private final String operationName;

	/**
	 * Constructor
	 * @param label label to go to when the value predicate is met
	 * @param predicate predicate on the node value
	 */
	private ConditionalOperation(
		final String label,
		final IntPredicate predicate,
		final String operationName) {
		this.targetLabel = label;
		this.valuePredicate = predicate;
		this.operationName = operationName;
	}

	/**
	 * Creates a JEZ conditional operation.
	 * @param label target label if the value is equal to 0
	 * @return the operation
	 */
	public static ConditionalOperation jez(final String label) {
		return new ConditionalOperation(label, value -> value == 0, "JEZ");
	}

	/**
	 * Creates a JNZ conditional operation.
	 * @param label target label if the value is not equal to 0
	 * @return the operation
	 */
	public static ConditionalOperation jnz(final String label) {
		return new ConditionalOperation(label, value -> value != 0, "JNZ");
	}

	/**
	 * Creates a JGZ conditional operation.
	 * @param label target label if the value is greater than 0
	 * @return the operation
	 */
	public static ConditionalOperation jgz(final String label) {
		return new ConditionalOperation(label, value -> value > 0, "JGZ");
	}

	/**
	 * Creates a JLZ conditional operation.
	 * @param label target label if the value is lower than 0
	 * @return the operation
	 */
	public static ConditionalOperation jlz(final String label) {
		return new ConditionalOperation(label, value -> value < 0, "JLZ");
	}

	@Override
	public Shift execute(final Node node) {
		if (node.testValue(this.valuePredicate)) {
			return this;
		} else {
			return Shift.NEXT;
		}
	}

	@Override
	public int update(final ToIntFunction<String> labelIndex, final int current, final int max) {
		return labelIndex.applyAsInt(this.targetLabel);
	}

	@Override
	public String toString() {
		return getClass().getSimpleName() + "[" + this.operationName + "]";
	}
}
