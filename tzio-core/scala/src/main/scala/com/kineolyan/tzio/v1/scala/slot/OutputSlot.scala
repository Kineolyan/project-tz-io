package com.kineolyan.tzio.v1.scala.slot

/**
  * Trait defining an environment output slot
  */
trait OutputSlot {
  def canWrite: Boolean
  def write(value: Int): OutputSlot
}
