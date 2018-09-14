package com.kineolyan.tzio.v1.scala.slot

class QueueSlot(values: List[Int]) extends InputSlot {
  override def canRead(): Boolean = !values.isEmpty

  override def read(): Unit = values.head
}
