package com.kineolyan.tzio.v1.scala.operations

import com.kineolyan.tzio.v1.scala.refs.{InputReference, OutputReference}

/**
  * Base class for all operations available on a node.
  */
abstract class Operation
case class MovOperation(in: InputReference, out: OutputReference) extends Operation {}
case class SavOperation(memoryIdx: Int) extends Operation {}
case class SwpOperation(memoryIdx: Int) extends Operation {}
case class AddOperation(in: InputReference) extends Operation {}
case class SubOperation(in: InputReference) extends Operation {}
case class NegOperation() extends Operation {}
case class LblOperation(label: String) extends Operation {}
case class JmpOperation(label: String) extends Operation {}
case class JezOperation(label: String) extends Operation {}
case class JnzOperation(label: String) extends Operation {}
case class JlzOperation(label: String) extends Operation {}
case class JgzOperation(label: String) extends Operation {}
case class JroOperation(in: InputReference) extends Operation {}
