package com.kineolyan.tzio.v1.scala.exec

import com.kineolyan.tzio.v1.scala.operations.MovOperation
import com.kineolyan.tzio.v1.scala.refs.{InSlotReference, InputReference, OutputReference}
import com.kineolyan.tzio.v1.scala.slot.InputSlot

class Execution(inputs: Array[Int], outputs: Array[Int], operations: Array[Any]) {

  /**
    * Produces a new context with the result of the execution
    * @param context
    * @return
    */
  def runCycle(context: Context): Context = {
    val operation = operations.apply(context.node.instruction)
    operation match {
      case MovOperation(in, out) => move(context, in, out)
      case _ => throw new IllegalArgumentException("Unsupported operation " + operation)
    }
  }

  private def move(context: Context, in: InputReference, out: OutputReference): Context = {
    val (value, idx, newInputs) = read(context, in)
  }

  private def read(context: Context, in: InputReference): (Int, Int, Array[InputSlot]) = {
    in match {
      case InSlotReference(idx) => {
        val slot = context.inputs.apply(idx - 1)

        val value = slot.canRead
      }
      case _ => throw new IllegalArgumentException("Unsupported input " + in)
    }
  }

  private def nextInstruction(current: Int, max: Int): Int = (current + 1) % max

}
