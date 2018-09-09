package com.kineolyan.tzio.v1.api.ref;

/**
 * Public facade to create references to inputs and outputs.
 */
public class References {

	private References() {}

	public static InputReferenceType inSlot(final int idx) {
		return new SlotReference(idx);
	}

	public static OutputReferenceType outSlot(final int idx) {
		return new SlotReference(idx);
	}

	public static InputReferenceType inAcc() {
		return new AccReference();
	}

	public static OutputReferenceType outAcc() {
		return new AccReference();
	}

	public static InputReferenceType value(final int value) {
		return new ValueReference(value);
	}

	public static InputReferenceType inNil() {
		return new NilReference();
	}

	public static OutputReferenceType outNil() {
		return new NilReference();
	}

}
