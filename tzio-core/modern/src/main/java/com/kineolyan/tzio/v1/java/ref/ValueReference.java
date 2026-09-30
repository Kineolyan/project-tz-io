package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;

import java.util.Objects;
import java.util.stream.IntStream;

/**
 * Reference to a static value.
 */
class ValueReference implements InputReference {

	/** Cache of common value */
	private static ValueReference[] CACHE;
	/** Minimal cached value (included) */
	private static int MIN_CACHED_VALUE = -128;
	/** Maximal cached value (excluded) */
	private static int MAX_CACHED_VALUE = 128;
	static {
		CACHE = IntStream.range(MIN_CACHED_VALUE, MAX_CACHED_VALUE)
			.mapToObj(ValueReference::new)
			.toArray(ValueReference[]::new);
	}

	/** Value held by the reference */
	private final int value;

	/**
	 * Constructor.
	 * @param value value held by the reference
	 */
	private ValueReference(final int value) {
		this.value = value;
	}

	/**
	 * Static constructor of a reference to a static value.
	 * @param value value held by the reference.
	 * @return the reference
	 */
	public static ValueReference of(final int value) {
		if (value >= -128 && value < 128) {
			return CACHE[value - MIN_CACHED_VALUE];
		} else {
			return new ValueReference(value);
		}
	}

	@Override
	public boolean canRead(Node node) {
		return true;
	}

	@Override
	public int readValue(final Node node) {
		return this.value;
	}

	@Override
	public String toString() {
		return this.getClass().getSimpleName() + "{" +
			"value=" + value +
			'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		ValueReference that = (ValueReference) o;
		return value == that.value;
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}
}
