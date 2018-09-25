package com.kineolyan.tzio.v1.scala.slot

/**
  * Implementation of an Input and Output holding a single value.
  */
class FilledSlot(val value: Int) extends InputSlot with OutputSlot {
  override def canRead: Boolean = true
  override def read(): (Int, InputSlot) = (value, new EmptySlot())

  override def canWrite: Boolean = false
  override def write(value: Int): OutputSlot = throw new RuntimeException("Cannot write into a filled slot")

  def canEqual(other: Any): Boolean = other.isInstanceOf[FilledSlot]
  override def equals(other: Any): Boolean = other match {
    case that: FilledSlot =>
      (that canEqual this) &&
        value == that.value
    case _ => false
  }

  override def hashCode(): Int = {
    val state = Seq(value)
    state.map(_.hashCode()).foldLeft(0)((a, b) => 31 * a + b)
  }
}
