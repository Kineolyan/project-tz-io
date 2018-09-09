package com.kineolyan.tzio.v1.api.ref;

public class NilReference implements InputReferenceType, OutputReferenceType {

	@Override
	public <R> R accept(InputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}

	@Override
	public <R> R accept(OutputReferenceVisitor<R> visitor) {
		return visitor.visit(this);
	}

}
