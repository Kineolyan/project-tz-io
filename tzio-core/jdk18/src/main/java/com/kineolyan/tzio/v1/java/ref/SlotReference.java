package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;

import java.util.Objects;
import java.util.stream.IntStream;

/**
 * Reference to a slot, either input or output.
 */
class SlotReference implements InputReference, OutputReference {

	/** Internal cache of references, commonly addressed */
	private static final SlotReference[] CACHE;
	static {
		CACHE = IntStream.range(1, 11)
			.mapToObj(SlotReference::new)
			.toArray(SlotReference[]::new);
	}

	/** 0-based index of the referenced slot */
	private final int slotIndex;

	/**
	 * Constructor
	 * @param slotId id of the slot - 1-based counted.
	 */
	public SlotReference(final int slotId) {
		this.slotIndex = slotId - 1;
	}

	/**
	 * Static constructor.
	 * @param id id of the slot - 1-based counted.
	 * @return a reference
	 */
	public static SlotReference of(final int id) {
		final int cacheIdx = id - 1;
		if (cacheIdx < CACHE.length) {
			return CACHE[cacheIdx];
		} else {
			return new SlotReference(id);
		}
	}

	/**
	 * Gets the referenced node input.
	 * @param node node to consider
	 * @return referenced input
	 */
	private InputSlot getInput(final Node node) {
		return node.getInput(this.slotIndex);
	}

	/**
	 * Gets the referenced node output.
	 * @param node node to consider
	 * @return referenced output
	 */
	private OutputSlot getOutput(final Node node) {
		return node.getOutput(this.slotIndex);
	}

	@Override
	public boolean canRead(final Node node) {
		return getInput(node).canRead();
	}

	@Override
	public int readValue(final Node node) {
		return getInput(node).read();
	}

	@Override
	public boolean canWrite(final Node node) {
		return getOutput(node).canWrite();
	}

	@Override
	public void writeValue(final Node node, final int value) {
		getOutput(node).write(value);
	}

	@Override
	public String toString() {
		return getClass().getSimpleName() + "{slot=" + (this.slotIndex + 1) + "}";
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		SlotReference that = (SlotReference) o;
		return slotIndex == that.slotIndex;
	}

	@Override
	public int hashCode() {
		return Objects.hash(slotIndex);
	}
}
