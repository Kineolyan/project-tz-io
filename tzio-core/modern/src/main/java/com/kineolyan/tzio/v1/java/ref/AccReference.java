package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;

/**
 * Reference to the node internal value.
 */
@RequiredArgsConstructor(access = AccessLevel.PRIVATE)
class AccReference implements InputReference, OutputReference {

	/** Singleton instance of this reference */
	public static AccReference INSTANCE = new AccReference();

	@Override
	public boolean canRead(final Node node) {
		return true;
	}

	@Override
	public int readValue(final Node node) {
		return node.getAccValue();
	}

	@Override
	public boolean canWrite(final Node node) {
		return true;
	}

	@Override
	public void writeValue(final Node node, final int value) {
		node.setAccValue(value);
	}

	@Override
	public String toString() {
		return "ACC";
	}
}
