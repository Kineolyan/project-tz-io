package com.kineolyan.tzio.v1.scala.slot

trait InputSlot {
  def canRead: Boolean
  def read(): (Int, InputSlot)
}
