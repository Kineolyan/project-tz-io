package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;

/**
 * Reference to the null source.
 */
@RequiredArgsConstructor(access = AccessLevel.PRIVATE)
class NilReference implements InputReference, OutputReference {

	/** Singleton instance of this reference */
	public static NilReference INSTANCE = new NilReference();

	@Override
	public boolean canRead(final Node node) {
		return true;
	}

	@Override
	public int readValue(final Node node) {
		return 0;
	}

	@Override
	public boolean canWrite(final Node node) {
		return true;
	}

	@Override
	public void writeValue(final Node node, final int value) {}

	@Override
	public String toString() {
		return "NIL";
	}
}
