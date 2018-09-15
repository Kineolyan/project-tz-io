package com.kineolyan.tzio.v1.scala.slot

trait OutputSlot {
  def canWrite: Boolean
  def write(value: Int): OutputSlot
}
