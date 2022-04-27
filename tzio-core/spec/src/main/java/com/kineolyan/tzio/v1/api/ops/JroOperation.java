package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;

public record JroOperation(InputReferenceType input) implements OperationType {

	@Override
	public <R> R accept(OperationVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
