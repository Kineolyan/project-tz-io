package com.kineolyan.tzio.v1.scala.exec

import com.kineolyan.tzio.v1.scala.operations.{MovOperation, Operation}
import com.kineolyan.tzio.v1.scala.refs.{InSlotReference, InputReference, OutSlotReference, OutputReference}
import com.kineolyan.tzio.v1.scala.slot.{InputSlot, OutputSlot}

class Execution(inputs: Array[Int], outputs: Array[Int], operations: Array[Operation]) {

  /**
    * Produces a new context with the result of the execution
    *
    * @param context execution context
    * @return changed context after the execution
    */
  def runCycle(context: Context): Context = {
    val operation = operations.apply(context.node.instruction)
    operation match {
      case MovOperation(in, out) => move(context, in, out)
      case _ => throw new IllegalArgumentException("Unsupported operation " + operation)
    }
  }

  private def move(context: Context, in: InputReference, out: OutputReference): Context = {
    read(context, in) match {
      case Some((value, newInputs)) =>
        write(context, value, out) match {
          case Some(newOutputs) =>
            val nextInstr = nextInstruction(context.node.instruction, operations.length)
            val newNode = context.node.copy(instruction = nextInstr)
            context.copy(node = newNode, inputs = newInputs, outputs = newOutputs)
          case None => context // No change to the context
        }
      case None => context // No change to do
    }
  }

  private def read(context: Context, in: InputReference): Option[(Int, Array[InputSlot])] = {
    in match {
      case InSlotReference(idx) =>
        val slot = context.inputs.apply(idx - 1)

        if (slot.canRead) {
          val (value, newSlot) = slot.read()
          val readInputs = context.inputs.clone()
          readInputs.update(idx, newSlot)

          Some((value, readInputs))
        } else {
          None
        }
      case _ => throw new IllegalArgumentException("Unsupported input " + in)
    }
  }

  private def write(context: Context, value: Int, out: OutputReference): Option[Array[OutputSlot]] = {
    out match {
      case OutSlotReference(idx) =>
        val slot = context.outputs.apply(idx - 1)

        if (slot.canWrite) {
          val newSlot = slot.write(value)
          val writtenOutputs = context.outputs.clone()
          writtenOutputs.update(idx, newSlot)

          Some(writtenOutputs)
        } else {
          None
        }
      case _ => throw new IllegalArgumentException("Unsupported input " + out)
    }
  }

  private def nextInstruction(current: Int, max: Int): Int = (current + 1) % max

}
