package com.kineolyan.tzio.v1.scala.slot

trait OutputSlot[T] {
  def canWrite(): Boolean
  def write(value: Int): T
}
