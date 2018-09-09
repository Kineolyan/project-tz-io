package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;

/**
 * Public facade for creating operations.
 */
public class Operations {

	private Operations() {}

	public static OperationType MOV(final InputReferenceType input, final OutputReferenceType output) {
		return new MovOperation(input, output);
	}

	public static OperationType SAV(final int slot) {
		return new SavOperation(slot);
	}

	public static OperationType SWP(final int slot) {
		return new SwpOperation(slot);
	}

	public static OperationType ADD(final InputReferenceType input) {
		return new AddOperation(input);
	}

	public static OperationType SUB(final InputReferenceType input) {
		return new SubOperation(input);
	}

	public static OperationType NEG() {
		return new NegOperation();
	}

	public static OperationType LABEL(final String label) {
		return new LabelOperation(label);
	}

	public static OperationType JMP(final String label) {
		return new JmpOperation(label);
	}

	public static OperationType JEZ(final String label) {
		return new JezOperation(label);
	}

	public static OperationType JNZ(final String label) {
		return new JnzOperation(label);
	}

	public static OperationType JLZ(final String label) {
		return new JlzOperation(label);
	}

	public static OperationType JGZ(final String label) {
		return new JgzOperation(label);
	}

	public static OperationType JRO(final InputReferenceType input) {
		return new JroOperation(input);
	}

}
