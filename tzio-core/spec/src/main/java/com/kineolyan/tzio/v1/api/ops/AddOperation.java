package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;

public class AddOperation implements OperationType {

	public final InputReferenceType input;


	public AddOperation(InputReferenceType input) {
		this.input = input;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
