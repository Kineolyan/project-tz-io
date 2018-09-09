package com.kineolyan.tzio.v1.api.ops;

/**
 * Description of an operation on a {@link Node}.
 */
public interface OperationType {

	<R> R accept(OperationVisitor<R> visitor);

}
