package com.kineolyan.tzio.v1.api.ops;

/**
 * Visitor of operation specification type.
 * @param <R> Type of result of this visitor.
 */
public interface OperationVisitor<R> {

	/**
	 * Visits a MOV operation
	 * @param movOperation operation
	 * @return the result of the visit
	 */
	R visit(MovOperation movOperation);
	/**
	 * Visits a SAV operation
	 * @param savOperation operation
	 * @return the result of the visit
	 */
	R visit(SavOperation savOperation);
	/**
	 * Visits a SWP operation
	 * @param swpOperation operation
	 * @return the result of the visit
	 */
	R visit(SwpOperation swpOperation);
	/**
	 * Visits a ADD operation
	 * @param addOperation operation
	 * @return the result of the visit
	 */
	R visit(AddOperation addOperation);
	/**
	 * Visits a SUB operation
	 * @param subOperation operation
	 * @return the result of the visit
	 */
	R visit(SubOperation subOperation);
	/**
	 * Visits a NEG operation
	 * @param negOperation operation
	 * @return the result of the visit
	 */
	R visit(NegOperation negOperation);
	/**
	 * Visits a LABEL operation
	 * @param labelOperation operation
	 * @return the result of the visit
	 */
	R visit(LabelOperation labelOperation);
	/**
	 * Visits a JMP operation
	 * @param jmpOperation operation
	 * @return the result of the visit
	 */
	R visit(JmpOperation jmpOperation);
	/**
	 * Visits a JEZ operation
	 * @param jezOperation operation
	 * @return the result of the visit
	 */
	R visit(JezOperation jezOperation);
	/**
	 * Visits a JNZ operation
	 * @param jnzOperation operation
	 * @return the result of the visit
	 */
	R visit(JnzOperation jnzOperation);
	/**
	 * Visits a JLZ operation
	 * @param jlzOperation operation
	 * @return the result of the visit
	 */
	R visit(JlzOperation jlzOperation);
	/**
	 * Visits a JGZ operation
	 * @param jgzOperation operation
	 * @return the result of the visit
	 */
	R visit(JgzOperation jgzOperation);
	/**
	 * Visits a JRO operation
	 * @param jroOperation operation
	 * @return the result of the visit
	 */
	R visit(JroOperation jroOperation);

	/**
	 * Visits a default operation.
	 * @param operation visited operation
	 * @return the result of the visit
	 */
	default R visit(OperationType operation) {
		throw new IllegalArgumentException("Unsupported operation " + operation);
	}
}
