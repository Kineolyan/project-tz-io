package com.kineolyan.tzio.v1.scala

import com.kineolyan.tzio.v1.scala.slot.{InputSlot, OutputSlot}

class Node(acc: Int, memory: Array[Int]) {

  def acc(value: Int): Node = new Node(value, memory)

  def move[U](input: InputSlot, output: OutputSlot[U]): Node = {
//    val value = input.read();
//    val
    this
  }

  def swap(idx: Int): Node = {
    val memoryValue = memory.apply(idx)
    var newMemory = memory.clone
    newMemory.update(idx, acc)
    new Node(memoryValue, newMemory)
  }
}

object Node {
  def ofSize(memorySize: Int): Node = {
    new Node(0, new Array[Int](memorySize))
  }
}
