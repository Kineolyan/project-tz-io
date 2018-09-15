package com.kineolyan.tzio.v1.scala

class Node(acc: Int, instruction: Int, memory: Array[Int]) {

  def copy(acc: Int = acc, instruction: Int = instruction, memory: Array[Int] = memory): Node =
    Node(acc, instruction, memory)

  def acc(value: Int): Node = copy(acc = value)

  def add(value: Int): Node = copy(acc = acc + value)
  def sub(value: Int): Node = copy(acc = acc - value)

  def bak(idx: Int): Node = {
    var newMemory = memory.clone
    newMemory.update(idx, acc)
    copy(memory = newMemory)
  }
  def swap(idx: Int): Node = {
    val newAcc = memory.apply(idx)
    var newMemory = memory.clone
    newMemory.update(idx, acc)
    copy(acc = newAcc, memory = newMemory)
  }

  def test(predicate: Int => Boolean) = predicate.apply(acc)

}

object Node {
  def ofSize(memorySize: Int): Node = {
    new Node(0, 0, new Array[Int](memorySize))
  }
}
