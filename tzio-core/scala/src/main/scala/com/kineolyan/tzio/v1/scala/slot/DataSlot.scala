package com.kineolyan.tzio.v1.scala.slot

abstract class DataSlot extends InputSlot with OutputSlot
case class FilledSlot(value: Int) extends DataSlot {
  override def canRead: Boolean = true
  override def read(): (Int, DataSlot) = (value, EmptySlot())

  override def canWrite: Boolean = false
  override def write(value: Int): DataSlot = throw new RuntimeException("Cannot write into a filled slot")
}
case class EmptySlot() extends DataSlot {
  override def canRead: Boolean = false
  override def read(): (Int, DataSlot) = throw new RuntimeException("Cannot read from a empty slot")

  override def canWrite: Boolean = true
  override def write(value: Int): DataSlot = FilledSlot(value)
}
