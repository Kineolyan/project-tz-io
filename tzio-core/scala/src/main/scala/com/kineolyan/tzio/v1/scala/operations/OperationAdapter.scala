package com.kineolyan.tzio.v1.scala.operations

import com.kineolyan.tzio.v1.api.ops._

object OperationAdapter extends OperationVisitor[Operation] {

  def convert(definition: OperationType): Operation = visit(definition)

  /**
    * Visits a MOV operation
    *
    * @param movOperation operation
    * @return the result of the visit
    */
  override def visit(movOperation: MovOperation): Any = ???

  /**
    * Visits a SAV operation
    *
    * @param savOperation operation
    * @return the result of the visit
    */
  override def visit(savOperation: SavOperation): Any = ???

  /**
    * Visits a SWP operation
    *
    * @param swpOperation operation
    * @return the result of the visit
    */
  override def visit(swpOperation: SwpOperation): Any = ???

  /**
    * Visits a ADD operation
    *
    * @param addOperation operation
    * @return the result of the visit
    */
  override def visit(addOperation: AddOperation): Any = ???

  /**
    * Visits a SUB operation
    *
    * @param subOperation operation
    * @return the result of the visit
    */
  override def visit(subOperation: SubOperation): Any = ???

  /**
    * Visits a NEG operation
    *
    * @param negOperation operation
    * @return the result of the visit
    */
  override def visit(negOperation: NegOperation): Any = ???

  /**
    * Visits a LABEL operation
    *
    * @param labelOperation operation
    * @return the result of the visit
    */
  override def visit(labelOperation: LabelOperation): Any = ???

  /**
    * Visits a JMP operation
    *
    * @param jmpOperation operation
    * @return the result of the visit
    */
  override def visit(jmpOperation: JmpOperation): Any = ???

  /**
    * Visits a JEZ operation
    *
    * @param jezOperation operation
    * @return the result of the visit
    */
  override def visit(jezOperation: JezOperation): Any = ???

  /**
    * Visits a JNZ operation
    *
    * @param jnzOperation operation
    * @return the result of the visit
    */
  override def visit(jnzOperation: JnzOperation): Any = ???

  /**
    * Visits a JLZ operation
    *
    * @param jlzOperation operation
    * @return the result of the visit
    */
  override def visit(jlzOperation: JlzOperation): Any = ???

  /**
    * Visits a JGZ operation
    *
    * @param jgzOperation operation
    * @return the result of the visit
    */
  override def visit(jgzOperation: JgzOperation): Any = ???

  /**
    * Visits a JRO operation
    *
    * @param jroOperation operation
    * @return the result of the visit
    */
  override def visit(jroOperation: JroOperation): Any = ???
}
