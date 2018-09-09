package com.kineolyan.tzio.v1.api.ref;

public class SlotReference implements InputReferenceType, OutputReferenceType {

	public final int slot;

	public SlotReference(int slot) {
		this.slot = slot;
	}

	@Override
	public <R> R accept(InputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}

	@Override
	public <R> R accept(OutputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}
}
