package com.kineolyan.tzio.v1.java;

import com.kineolyan.tzio.v1.java.ops.Operation;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Class executing operations on a given node.
 */
public class NodeExecution {

	/** Node on which operations are executed */
	private final Node node;
	/** List of operations to execute on the node */
	private final List<Operation> operations;
	/** Index structure of labeled operations, as {@code [<operation label>] = <operation index>} */
	private final Map<String, Integer> labels;
	/** Index of the current operation */
	private int stepIdx;

	/**
	 * Constructor
	 * @param node node on which operations are executed
	 * @param operations operation to execute on the node
	 */
	public NodeExecution(
		final Node node,
		final List<Operation> operations) {
		this.node = node;
		this.operations = new ArrayList<>();
		this.labels = new HashMap<>();
		this.stepIdx = 0;

		for (final Operation operation : operations) {
			if (operation.label() != null) {
				this.labels.put(operation.label(), this.operations.size());
			} else {
				this.operations.add(operation);
			}
		}
	}

	/**
	 * Runs the operation.
	 * <p>
	 *   It is possible for the same operation to be executed many times.
	 * </p>
	 */
	public void runStep() {
		final Operation operation = this.operations.get(this.stepIdx);
		final Operation.Shift nextOperation = operation.execute(this.node);
		this.stepIdx = nextOperation.update(this::getLabelOperationIdx, this.stepIdx, this.operations.size());
	}

	/**
	 * Gets the index of the label in the operation
	 * @param targetLabel label to reach
	 * @return the index of the operation to run after the label
	 */
	public int getLabelOperationIdx(final String targetLabel) {
		final int index = this.labels.computeIfAbsent(
			targetLabel,
			label -> { throw new IllegalArgumentException("No operation named " + label); });
		return index < this.operations.size() ? index : 0;
	}

}
