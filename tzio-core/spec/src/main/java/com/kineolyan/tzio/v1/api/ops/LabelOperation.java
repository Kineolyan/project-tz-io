package com.kineolyan.tzio.v1.api.ops;

public record LabelOperation(String label) implements OperationType {

	@Override
	public <R> R accept(OperationVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
