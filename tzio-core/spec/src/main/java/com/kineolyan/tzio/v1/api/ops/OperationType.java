package com.kineolyan.tzio.v1.api.ops;

/**
 * Description of an operation on a {@link Node}.
 */
public sealed interface OperationType permits AddOperation, JezOperation, JgzOperation, JlzOperation, JmpOperation, JnzOperation, JroOperation, LabelOperation, MovOperation, NegOperation, SavOperation, SubOperation, SwpOperation {

	<R> R accept(OperationVisitor<R> visitor);

}
