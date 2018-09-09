package com.kineolyan.tzio.v1.api.ref;

/**
 * Representation of a reference to an output.
 * <p>
 *   The reference can check that the output can be written and can perform the write.
 * </p>
 */
public interface OutputReferenceType {

	<R> R accept(OutputReferenceVisitor<R> visitor);

}
