package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

/**
 * Operation saving the current node value into one of its internal memory slots
 */
class SaveOperation implements Operation {

	/** Index of the internal node memory slot */
	private final int slot;

	/**
	 * Constructor.
	 * @param slot Index of the internal node memory slot
	 */
	public SaveOperation(int slot) {
		this.slot = slot;
	}

	@Override
	public Shift execute(final Node node) {
		node.bakValue(this.slot);
		return Shift.NEXT;
	}
}
