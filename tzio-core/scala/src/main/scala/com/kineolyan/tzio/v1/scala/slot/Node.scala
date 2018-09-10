package com.kineolyan.tzio.v1.scala.slot

class Node(acc: Int, memory: Array[Int]) {
  def acc(value: Int): Node = new Node(value, memory)
  def swap(idx: Int): Node = {
    var memoryValue: Int = memory.apply(idx)
    memory.update(idx, acc)
    new Node(memoryValue, memory)
  }
}
