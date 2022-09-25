package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;

/**
 * Operation swapping the internal node value with one of its memory value.
 */
class SwapOperation implements Operation {

	/** Index of the internal node memory slot */
	private final int slot;

	/**
	 * Constructor.
	 * @param slot Index of the internal node memory slot
	 */
	public SwapOperation(int slot) {
		this.slot = slot;
	}

	@Override
	public Shift execute(final Node node) {
		node.swapValue(this.slot);
		return Shift.NEXT;
	}
}
