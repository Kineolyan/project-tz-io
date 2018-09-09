package com.kineolyan.tzio.v1.api.ops;

public class JgzOperation implements OperationType {

	public final String label;

	public JgzOperation(String label) {
		this.label = label;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
