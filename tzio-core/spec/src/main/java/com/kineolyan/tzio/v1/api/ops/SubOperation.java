package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;

public class SubOperation implements OperationType {

	public final InputReferenceType input;


	public SubOperation(InputReferenceType input) {
		this.input = input;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
