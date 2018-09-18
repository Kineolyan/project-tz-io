package com.kineolyan.tzio.v1.scala.env

class EnvSlots(val slots: Array[Any], val inputs: Array[Int], val outputs: Array[Int]) {}

object EnvSlots {
  def empty(): EnvSlots = new EnvSlots(Array(), Array(), Array())
}
