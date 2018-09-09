package com.kineolyan.tzio.v1.api.ops;

public class LabelOperation implements OperationType {

	public final String label;

	public LabelOperation(String label) {
		this.label = label;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
