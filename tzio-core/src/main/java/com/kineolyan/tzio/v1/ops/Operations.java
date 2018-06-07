package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.ref.InputReference;
import com.kineolyan.tzio.v1.ref.OutputReference;

/**
 * Public facade for creating operations.
 */
public class Operations {

	private Operations() {}

	public static Operation MOV(final InputReference input, final OutputReference output) {
		return new MovOperation(input, output);
	}

	public static Operation SAV(final int slot) {
		return new SaveOperation(slot);
	}

	public static Operation SWP(final int slot) {
		return new SwapOperation(slot);
	}

	public static Operation ADD(final InputReference input) {
		return IncrementOperation.add(input);
	}

	public static Operation SUB(final InputReference input) {
		return IncrementOperation.sub(input);
	}

	public static Operation NEG() {
		return NegOperation.INSTANCE;
	}

	public static Operation LABEL(final String label) {
		return new LabelOperation(label);
	}

	public static Operation JMP(final String label) {
		return new JmpOperation(label);
	}

	public static Operation JEZ(final String label) {
		return ConditionalOperation.jez(label);
	}

	public static Operation JNZ(final String label) {
		return ConditionalOperation.jnz(label);
	}

	public static Operation JLZ(final String label) {
		return ConditionalOperation.jlz(label);
	}

	public static Operation JGZ(final String label) {
		return ConditionalOperation.jgz(label);
	}

	public static Operation JRO(final InputReference input) {
		return new JroOperation(input);
	}

}
