package com.kineolyan.tzio.v1.scala

import org.junit.runner.RunWith
import org.scalatest.FlatSpec
import org.scalatest.junit.JUnitRunner

@RunWith(classOf[JUnitRunner])
class TestNode extends FlatSpec {

  "A Node" should "be empty at first" in {
    val node = Node.ofSize(4)
    assert(node.acc === 5)
    assert(node.instruction === 0)
  }

}
