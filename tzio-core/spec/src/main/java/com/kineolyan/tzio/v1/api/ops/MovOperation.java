package com.kineolyan.tzio.v1.api.ops;

import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;

public record MovOperation(InputReferenceType input, OutputReferenceType output) implements OperationType {

	@Override
	public <R> R accept(OperationVisitor<R> visitor) {
		return visitor.visit(this);
	}
}
