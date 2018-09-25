package com.kineolyan.tzio.v1.scala.slot

/**
  * Implementation of an input slot that store multiple data to process
  * @param values list of available values ready to be consumed
  */
class QueueSlot(values: List[Int]) extends InputSlot {
  override def canRead: Boolean = values.nonEmpty

  override def read(): (Int, QueueSlot) = (values.head, new QueueSlot(values.tail))

  def consume(value: Int): QueueSlot =
    new QueueSlot(values ++ List(value))
}
