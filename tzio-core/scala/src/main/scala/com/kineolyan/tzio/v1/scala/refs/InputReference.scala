package com.kineolyan.tzio.v1.scala.refs

/**
  * Base class defining a source of input for an operation
  */
abstract class InputReference
case class InSlotReference(index: Int) extends InputReference {}
case class ValueReference(value: Int) extends InputReference {}
case class InAccReference() extends InputReference {}
case class InNilReference() extends InputReference {}
