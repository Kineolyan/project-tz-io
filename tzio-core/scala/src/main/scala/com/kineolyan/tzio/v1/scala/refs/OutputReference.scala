package com.kineolyan.tzio.v1.scala.refs

/**
  * Base class defining a source of output for an operation
  */
abstract class OutputReference
case class OutSlotReference(index: Int) extends OutputReference {}
case class OutAccReference() extends OutputReference {}
case class OutNilReference() extends OutputReference {}




