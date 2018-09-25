package com.kineolyan.tzio.v1.scala.slot

/**
  * Implementation of an Input and Output holding a single value.
  */
class FilledSlot(value: Int) extends InputSlot with OutputSlot {
  override def canRead: Boolean = true
  override def read(): (Int, InputSlot) = (value, new EmptySlot())

  override def canWrite: Boolean = false
  override def write(value: Int): OutputSlot = throw new RuntimeException("Cannot write into a filled slot")
}
