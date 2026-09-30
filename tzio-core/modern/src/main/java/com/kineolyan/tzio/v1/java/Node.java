package com.kineolyan.tzio.v1.java;

import com.kineolyan.tzio.v1.java.ref.InputReference;
import com.kineolyan.tzio.v1.java.ref.OutputReference;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;

import java.util.function.IntPredicate;

/**
 * Implementation of a node in the TZ-IO environment.
 * <p>
 *   A node is composed of a series of inputs it can read, outputs it can write into,
 *   a variable length internal memory and a given current value.
 * </p>
 */
public class Node {

	/** Node inputs */
	private final InputSlot[] inputSlots;
	/** Node outputs */
	private final OutputSlot[] outputSlots;
	/** Node internal value */
	private int accValue;
	/** Node memory slots, usable for value keeping */
	private final int[] memorySlots;

	/**
	 * Constructor.
	 * @param memorySize size of the internal memory
	 * @param inputSlots node inputs
	 * @param outputSlots node outputs
	 */
	public Node(
		final int memorySize,
		final InputSlot[] inputSlots,
		final OutputSlot[] outputSlots) {
		this.memorySlots = new int[memorySize];
		this.accValue = 0;
		this.inputSlots = inputSlots;
		this.outputSlots = outputSlots;
	}

	/**
	 * Reads the node value.
	 * @return node value
	 */
	public int getAccValue() {
		return this.accValue;
	}

	/**
	 * Sets the node value to a given input
	 * @param value new value for the node
	 */
	public void setAccValue(final int value) {
		this.accValue = value;
	}

	/**
	 * Access a given input of the node.
	 * @param idx 0-based index of the input in the node.
	 * @return the slot
	 */
	public InputSlot getInput(final int idx) {
		return this.inputSlots[idx];
	}

	/**
	 * Access a given output of the node.
	 * @param idx 0-based index of the output in the node.
	 * @return the slot
	 */
	public OutputSlot getOutput(final int idx) {
		return this.outputSlots[idx];
	}

	/**
	 * Gets the value stored in the indexed memory slot.
	 * @param slot slot index
	 * @return the memory value
	 */
	public int getMemoryValue(final int slot) {
		return this.memorySlots[slot - 1];
	}

	// Operations

	/**
	 * Moves the value of input into the output.
	 * @param from input to read for a value
	 * @param to output to write with a value
	 */
	public final void moveValue(final InputReference from, final OutputReference to) {
		final int value = from.readValue(this);
		to.writeValue(this, value);
	}

	/**
	 * Adds the value of the input to the internal value.
	 * @param input input to read for a value
	 */
	public final void addValue(final InputReference input) {
		this.accValue += input.readValue(this);
	}

	/**
	 * Subtracts the value of the input of the internal value.
	 * @param input input to read for a value
	 */
	public final void subValue(final InputReference input) {
		this.accValue -= input.readValue(this);
	}

	/**
	 * Negates the internal value.
	 */
	public final void negate() {
		this.accValue = -this.accValue;
	}

	/**
	 * Saves the internal value to the indexed memory slot.
	 * @param memorySlot index of the memory slot where the value is saved.
	 */
	public final void bakValue(final int memorySlot) {
		this.memorySlots[memorySlot - 1] = this.accValue;
	}

	/**
	 * Swaps the internal value with the indexed memory slot.
	 * @param memorySlot index of the memory slot.
	 */
	public final void swapValue(final int memorySlot) {
		final int index = memorySlot - 1;
		final int swp = this.accValue;
		this.accValue = this.memorySlots[index];
		this.memorySlots[index] = swp;
	}

	/**
	 * Tests the internal value against a predicate.
	 * @param predicate predicate on the value.
	 * @return result of the predicate
	 */
	public final boolean testValue(final IntPredicate predicate) {
		return predicate.test(this.accValue);
	}

}
