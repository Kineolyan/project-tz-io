package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ref.InputReference;

import java.util.function.ToIntFunction;

/**
 * Conditional operation offsetting to the "next" operation according to the input value.
 */
class JroOperation implements Operation {

	/** Input to read for the shift increment */
	private final InputReference input;

	/**
	 * Constructor.
	 * @param input input to read for the shift value.
	 */
	public JroOperation(final InputReference input) {
		this.input = input;
	}

	@Override
	public Shift execute(final Node node) {
		if (this.input.canRead(node)) {
			final int value = this.input.readValue(node);
			return JroShift.create(value);
		} else {
			return Shift.STAY;
		}
	}

	/**
	 * Special increment shifting by a given value.
	 */
	private static class JroShift implements Operation.Shift {

		/** Increment to apply to the operation */
		private final int increment;

		/**
		 * Constructor.
		 * @param increment operation increment
		 */
		private JroShift(final int increment) {
			this.increment = increment;
		}

		/**
		 * Creates the appropriate shift according to the increment.
		 * @param increment increment to apply
		 * @return the increment
		 */
		public static Operation.Shift create(final int increment) {
			if (increment == 0) {
				return Shift.STAY;
			} else if (increment == 1) {
				return Shift.NEXT;
			} else {
				return new JroShift(increment);
			}
		}

		@Override
		public int update(final ToIntFunction<String> labelIndex, final int current, final int max) {
			int target = current + this.increment;
			if (target < 0) {
				while (target < 0) {
					target += max;
				}
			} else {
				target %= max;
			}
			return target;
		}

	}
}
