package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;

public class MovOperation implements OperationType {

	public final InputReferenceType input;
	public final OutputReferenceType output;


	public MovOperation(final InputReferenceType input, final OutputReferenceType output) {
		this.input = input;
		this.output = output;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}
}
