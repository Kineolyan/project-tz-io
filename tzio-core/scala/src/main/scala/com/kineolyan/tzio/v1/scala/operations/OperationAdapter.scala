package com.kineolyan.tzio.v1.scala.operations

import com.kineolyan.tzio.v1.api._
import com.kineolyan.tzio.v1.scala.refs.{InputAdapter, OutputAdapter}

/**
  * Object adapting the definition of an operation to their actual types in the Scala core
  */
object OperationAdapter extends ops.OperationVisitor[Operation] {

  def convert(definition: ops.OperationType): Operation =
    definition.accept(this)

  override def visit(movOperation: ops.MovOperation): Operation =
    MovOperation(
      InputAdapter.convert(movOperation.input),
      OutputAdapter.convert(movOperation.output))

  override def visit(savOperation: ops.SavOperation): Operation =
    SavOperation(savOperation.slot)

  override def visit(swpOperation: ops.SwpOperation): Operation =
    SwpOperation(swpOperation.slot)

  override def visit(addOperation: ops.AddOperation): Operation =
    AddOperation(
      InputAdapter.convert(addOperation.input))

  override def visit(subOperation: ops.SubOperation): Operation =
    SubOperation(
      InputAdapter.convert(subOperation.input))

  override def visit(negOperation: ops.NegOperation): Operation = NegOperation()

  override def visit(labelOperation: ops.LabelOperation): Operation =
    LblOperation(labelOperation.label)

  override def visit(jmpOperation: ops.JmpOperation): Operation =
    JmpOperation(jmpOperation.label)

  override def visit(jezOperation: ops.JezOperation): Operation =
    JezOperation(jezOperation.label)

  override def visit(jnzOperation: ops.JnzOperation): Operation =
    JnzOperation(jnzOperation.label)

  override def visit(jlzOperation: ops.JlzOperation): Operation =
    JlzOperation(jlzOperation.label)

  override def visit(jgzOperation: ops.JgzOperation): Operation =
    JgzOperation(jgzOperation.label)

  override def visit(jroOperation: ops.JroOperation): Operation =
    JroOperation(
      InputAdapter.convert(jroOperation.input))
}
