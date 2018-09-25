package com.kineolyan.tzio.v1.scala.slot

import org.scalatest.FlatSpec

class TestEmptySlot  extends FlatSpec {

  behavior of "An Empty node"

  it should "not be readable" in {
    val slot = new EmptySlot()
    assert(slot.canRead === false)
  }

  it should "throw on reading" in {
    val slot = new EmptySlot()
    assertThrows[RuntimeException] {
      slot.read()
    }
  }

  it should "be writable" in {
    val slot = new EmptySlot()
    assert(slot.canWrite === true)
  }

  it should "create a new slot" in {
    val slot = new EmptySlot()
    val newSlot = slot.write(12)
    assert(slot !== newSlot)
    assert(newSlot === new FilledSlot(12))
  }

}
