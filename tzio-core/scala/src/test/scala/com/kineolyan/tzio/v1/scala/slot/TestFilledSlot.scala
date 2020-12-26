package com.kineolyan.tzio.v1.scala.slot

import org.scalatest.flatspec.AnyFlatSpec

class TestFilledSlot extends AnyFlatSpec {

  behavior of "A filled node"

  it should "be readable" in {
    val slot = new FilledSlot(4)
    assert(slot.canRead === true)
  }

  it should "not be writable" in {
    val slot = new FilledSlot(4)
    assert(slot.canWrite === false)
  }

  it should "create a new slot at reading" in {
    val slot = new FilledSlot(12)
    val (value, newSlot) = slot.read()
    assert(value === 12)
    assert(slot !== newSlot)
    assert(newSlot === new EmptySlot())
  }

  it should "throw on writing" in {
    val slot = new FilledSlot(75)
    assertThrows[RuntimeException] {
      slot.write(47520)
    }
  }

}
