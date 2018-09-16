package com.kineolyan.tzio.v1.scala.exec

import com.kineolyan.tzio.v1.scala.Node
import com.kineolyan.tzio.v1.scala.slot.{InputSlot, OutputSlot}

class Context(val node: Node, val inputs: Array[InputSlot], val outputs: Array[OutputSlot]) {

  def copy(node: Node = node, inputs: Array[InputSlot] = inputs, outputs: Array[OutputSlot] = outputs) =
    new Context(node, inputs, outputs)

}
