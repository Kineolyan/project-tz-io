package com.kineolyan.tzio.v1.scala.slot

/**
  * Trait defining an environment input slot
  */
trait InputSlot {
  def canRead: Boolean
  def read(): (Int, InputSlot)
}
