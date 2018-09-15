package com.kineolyan.tzio.v1.scala.refs

abstract class OutputReference
case class OutSlotReference(index: Int) extends OutputReference {}
case class OutAccReference() extends OutputReference {}
case class OutNilReference() extends OutputReference {}




