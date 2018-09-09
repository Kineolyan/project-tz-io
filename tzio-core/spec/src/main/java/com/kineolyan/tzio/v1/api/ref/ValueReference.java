package com.kineolyan.tzio.v1.api.ref;

public class ValueReference implements InputReferenceType {

	public final int value;

	public ValueReference(int value) {
		this.value = value;
	}

	@Override
	public <R> R accept(InputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
