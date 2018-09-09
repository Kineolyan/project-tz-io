package com.kineolyan.tzio.v1.api.ref;

public interface InputReferenceVisitor<R> {

	R visit(SlotReference ref);
	R visit(AccReference ref);
	R visit(ValueReference ref);
	R visit(NilReference ref);

	default R visit(InputReferenceType type) {
		throw new IllegalStateException("Unsupported input " + type);
	}

}
