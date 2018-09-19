package com.kineolyan.tzio.v1.scala.env

class EnvSlots(val slots: Array[Any], val inputs: Array[Int], val outputs: Array[Int]) {

  def copy(slots: Array[Any] = slots, inputs: Array[Int] = inputs, outputs: Array[Int] = outputs) =
    new EnvSlots(slots, inputs, outputs)

}

object EnvSlots {
  def empty(): EnvSlots = new EnvSlots(Array(), Array(), Array())
}
