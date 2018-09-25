package com.kineolyan.tzio.v1.scala.env

import com.kineolyan.tzio.v1.scala.slot.{InputSlot, OutputSlot}

/**
  * Structure holding the environment slots and the indexes of global inputs and outputs.
  * @param slots environment slots
  * @param inputs 0-based indexes of slots used as inputs of the environment
  * @param outputs 0-based indexes of slots used as outputs of the environment
  */
class EnvSlots(val slots: Array[Any], val inputs: Array[Int], val outputs: Array[Int]) {

  def copy(slots: Array[Any] = slots, inputs: Array[Int] = inputs, outputs: Array[Int] = outputs) =
    new EnvSlots(slots, inputs, outputs)

  def input(idx: Int): InputSlot =
    slots.apply(idx) match {
      case s: InputSlot => s
      case slot => throw new IllegalStateException("Expecting " + slot + " to be an input")
    }

  def output(idx: Int): OutputSlot =
    slots.apply(idx) match {
      case s: OutputSlot => s
      case slot => throw new IllegalStateException("Expecting " + slot + " to be an output")
    }

}

object EnvSlots {
  def empty(): EnvSlots = new EnvSlots(Array(), Array(), Array())
}
