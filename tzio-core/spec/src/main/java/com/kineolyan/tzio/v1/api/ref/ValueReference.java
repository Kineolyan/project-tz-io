package com.kineolyan.tzio.v1.api.ref;

public record ValueReference(int value) implements InputReferenceType {

	@Override
	public <R> R accept(InputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
