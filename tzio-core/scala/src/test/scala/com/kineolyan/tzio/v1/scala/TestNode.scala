package com.kineolyan.tzio.v1.scala

import org.scalatest.FlatSpec

class TestNode extends FlatSpec {

  behavior of "A Node"

  it should "be empty at construction" in {
    val node = Node.ofSize(4)
    assert(node.acc === 0)
    assert(node.instruction === 0)
  }

}
