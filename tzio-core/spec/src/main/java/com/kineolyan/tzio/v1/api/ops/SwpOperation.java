package com.kineolyan.tzio.v1.api.ops;

public record SwpOperation(int slot) implements OperationType {

	@Override
	public <R> R accept(OperationVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
