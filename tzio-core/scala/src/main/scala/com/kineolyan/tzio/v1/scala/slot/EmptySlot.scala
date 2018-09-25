package com.kineolyan.tzio.v1.scala.slot

/**
  * Implementation of an Input and Output without any value.
  */
class EmptySlot() extends InputSlot with OutputSlot {
  override def canRead: Boolean = false
  override def read(): (Int, InputSlot) = throw new RuntimeException("Cannot read from a empty slot")

  override def canWrite: Boolean = true
  override def write(value: Int): OutputSlot = new FilledSlot(value)

  def canEqual(other: Any): Boolean = other.isInstanceOf[EmptySlot]
  override def equals(other: Any): Boolean = other match {
    case that: EmptySlot => that canEqual this
    case _ => false
  }

  override def hashCode(): Int = 1

}
