package com.kineolyan.tzio.v1.scala.slot

class QueueSlot(values: List[Int]) extends InputSlot {
  override def canRead: Boolean = values.nonEmpty

  override def read(): (Int, QueueSlot) = (values.head, new QueueSlot(values.tail))
}
