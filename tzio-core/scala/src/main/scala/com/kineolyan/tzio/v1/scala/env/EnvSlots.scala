package com.kineolyan.tzio.v1.scala.env

class EnvSlots(slots: Array[Any], inputs: Array[Int], outputs: Array[Int]) {}

object EnvSlots {
  def empty(): EnvSlots = EnvSlots(Array(), Array(), Array())
}
