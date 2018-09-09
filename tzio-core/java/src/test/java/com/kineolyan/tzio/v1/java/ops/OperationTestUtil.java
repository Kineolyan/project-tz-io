/*
 * (C) ActiveViam FS 2013-2018
 * ALL RIGHTS RESERVED. This material is the CONFIDENTIAL and PROPRIETARY
 * property of Quartet Financial Systems Limited. Any unauthorized use,
 * reproduction or transfer of this material is strictly prohibited
 */
package com.kineolyan.tzio.v1.java.ops;

import java.util.stream.IntStream;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.slot.DataSlot;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;
import org.assertj.core.api.AbstractAssert;

public class OperationTestUtil {

	/**
	 * Creates a default node with 8 memory slots, 3 input slots and 2 output slots.
	 * @return the created node
	 */
	static Node defaultNode() {
		return new Node(
				8,
				IntStream.range(0, 3).mapToObj(i -> new DataSlot()).toArray(InputSlot[]::new),
				IntStream.range(0, 2).mapToObj(i -> new DataSlot()).toArray(OutputSlot[]::new));
	}

	public static ShiftAssert assertThat(Operation.Shift shift) {
		return new ShiftAssert(shift);
	}

	static <T extends InputSlot> T getInput(final Node node, final int index) {
		return (T) node.getInput(index - 1);
	}

	static <T extends OutputSlot> T getOutput(final Node node, final int index) {
		return (T) node.getOutput(index - 1);
	}

	static class ShiftAssert extends AbstractAssert<ShiftAssert, Operation.Shift> {

		public ShiftAssert(final Operation.Shift actual) {
			super(actual, ShiftAssert.class);
		}

		public ShiftAssert shiftToNext() {
			isNotNull();

			if (this.actual != Operation.Shift.NEXT) {
				failWithMessage("Expected shift to next but got " + this.actual);
			}

			return this;
		}
	}

}

