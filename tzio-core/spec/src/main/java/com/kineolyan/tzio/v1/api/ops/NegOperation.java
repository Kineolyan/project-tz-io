package com.kineolyan.tzio.v1.api.ops;

public class NegOperation implements OperationType {

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
