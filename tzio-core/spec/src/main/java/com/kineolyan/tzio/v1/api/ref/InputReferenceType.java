package com.kineolyan.tzio.v1.api.ref;

/**
 * Representation of a reference to an input.
 * <p>
 *   The reference can check that the input can provide a value and can read it.
 * </p>
 */
public sealed interface InputReferenceType permits SlotReference, AccReference, ValueReference, NilReference{

	<R> R accept(InputReferenceVisitor<R> visitor);

}
