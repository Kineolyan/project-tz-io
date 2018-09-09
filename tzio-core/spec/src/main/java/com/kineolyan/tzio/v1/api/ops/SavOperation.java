package com.kineolyan.tzio.v1.api.ops;

public class SavOperation implements OperationType {

	public final int slot;

	public SavOperation(final int slot) {
		this.slot = slot;
	}

	@Override
	public <R> R accept(OperationVisitor<R>visitor) {
		return visitor.visit(this);
	}

}
