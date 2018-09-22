package com.kineolyan.tzio.v1.scala.exec

import com.kineolyan.tzio.v1.scala.Node
import com.kineolyan.tzio.v1.scala.operations._
import com.kineolyan.tzio.v1.scala.refs._
import com.kineolyan.tzio.v1.scala.slot.{InputSlot, OutputSlot}

class Execution(
                 val inputs: Array[Int],
                 val outputs: Array[Int],
                 operations: Array[Operation],
                 labelMapping: Map[String, Int]) {

  def this(inputs: Array[Int], outputs: Array[Int], operations: Array[Operation]) =
    this(
      inputs,
      outputs,
      operations.filter({
        case LblOperation(_) => false
        case _ => true
      }),
      operations.foldLeft(Map[String, Int]())((acc, operation) => operation match {
        case LblOperation(label) => acc + (label -> acc.size)
        case _ => acc
      }))

  /**
    * Produces a new context with the result of the execution
    *
    * @param context execution context
    *
    * @return changed context after the execution
    */
  def runCycle(context: Context): Context = {
    val operation = operations.apply(context.node.instruction)
    operation match {
      case MovOperation(in, out) => move(context, in, out)
      case SavOperation(memoryIdx) => save(context, memoryIdx)
      case SwpOperation(memoryIdx) => swap(context, memoryIdx)
      case AddOperation(in) => add(context, in)
      case SubOperation(in) => sub(context, in)
      case NegOperation() => neg(context)
      case LblOperation(label) =>
        throw new IllegalStateException("Label operations must have been filtered at insertion. Read: " + operation)
      case JmpOperation(label) => jump(context, label)
      case JezOperation(label) => jez(context, label)
      case JnzOperation(label) => jnz(context, label)
      case JlzOperation(label) => jlz(context, label)
      case JgzOperation(label) => jgz(context, label)
      case JroOperation(in) => jro(context, in)
    }
  }

  private def move(context: Context, in: InputReference, out: OutputReference): Context = {
    read(context, in) match {
      case Some((value, newInputs)) =>
        write(context, value, out) match {
          case Some(newContext) =>
            val nextInstr = nextInstruction(newContext)
            val newNode = newContext.node.copy(instruction = nextInstr)
            newContext.copy(node = newNode, inputs = newInputs)
          case None => context // No change to the context
        }
      case None => context // No change to do
    }
  }

  private def save(context: Context, idx: Int): Context = {
    val updatedNode = context.node.bak(idx)
    val shiftedNode = shiftNode(updatedNode, context)
    context.copy(node = shiftedNode)
  }

  private def swap(context: Context, idx: Int): Context = {
    val updatedNode = context.node.swap(idx)
    val shiftedNode = shiftNode(updatedNode, context)
    context.copy(node = shiftedNode)
  }

  private def add(context: Context, in: InputReference): Context = {
    read(context, in) match {
      case Some((value, newInputs)) => {
        val updatedNode = context.node.add(value)
        val shiftedNode = shiftNode(updatedNode, context)
        context.copy(node = shiftedNode, inputs = newInputs)
      }
      case _ => context
    }
  }

  private def sub(context: Context, in: InputReference): Context = {
    read(context, in) match {
      case Some((value, newInputs)) => {
        val updatedNode = context.node.sub(value)
        val shiftedNode = shiftNode(updatedNode, context)
        context.copy(node = shiftedNode, inputs = newInputs)
      }
      case _ => context
    }
  }

  private def neg(context: Context): Context = {
    val updatedNode = context.node.neg();
    val shiftedNode = shiftNode(updatedNode, context)
    context.copy(node = shiftedNode)
  }

  private def jump(context: Context, label: String): Context = jumpIf(context, label, _ => true)
  private def jez(context: Context, label: String): Context = jumpIf(context, label, acc => acc == 0)
  private def jnz(context: Context, label: String): Context = jumpIf(context, label, acc => acc != 0)
  private def jlz(context: Context, label: String): Context = jumpIf(context, label, acc => acc > 0)
  private def jgz(context: Context, label: String): Context = jumpIf(context, label, acc => acc < 0)

  private def jro(context: Context, in: InputReference): Context = {
    read(context, in) match {
      case Some((value, newInputs)) => {
        var nextInstr = context.node.instruction + value
        while (nextInstr < 0) {
          nextInstr += operations.length
        }
        nextInstr %= operations.length
        val shiftedNode = context.node.copy(instruction = nextInstr)
        context.copy(node = shiftedNode, inputs = newInputs)
      }
      case _ => context
    }
  }

  /**
    * Reads a value from an input if possible.
    *
    * @param context execution context
    * @param in slot to read
    *
    * @return the new input slots if modified
    */
  private def read(context: Context, in: InputReference): Option[(Int, Array[InputSlot])] = {
    in match {
      case InSlotReference(idx) =>
        val slot = context.inputs.apply(idx - 1)

        if (slot.canRead) {
          val (value, newSlot) = slot.read()
          val readInputs = context.inputs.clone()
          readInputs.update(idx - 1, newSlot)

          Some((value, readInputs))
        } else {
          None
        }
      case ValueReference(value: Int) => Some((value, context.inputs))
      case InAccReference() => Some((context.node.acc, context.inputs))
      case InNilReference() => Some((0, context.inputs))
    }
  }

  /**
    * Writes a value into a slot if possible.
    *
    * @param context execution context
    * @param value value to write
    * @param out slot to be fed by with the value
    *
    * @return the new output slots if modified
    */
  private def write(context: Context, value: Int, out: OutputReference): Option[Context] = {
    out match {
      case OutSlotReference(idx) =>
        val slot = context.outputs.apply(idx - 1)

        if (slot.canWrite) {
          val newSlot = slot.write(value)
          val writtenOutputs = context.outputs.clone()
          writtenOutputs.update(idx - 1, newSlot)

          Some(context.copy(outputs = writtenOutputs))
        } else {
          None
        }
      case OutAccReference() => {
        val writtenNode = context.node.acc(value)
        Some(context.copy(node = writtenNode))
      }
      case OutNilReference() => Some(context) // Void writing
    }
  }

  private def shiftNode(node: Node, context: Context): Node = {
    val nextInstr = nextInstruction(context)
    node.copy(instruction = nextInstr)
  }

  private def nextInstruction(context: Context): Int =
    _nextInstruction(context.node.instruction, operations.length)

  private def _nextInstruction(current: Int, max: Int): Int = (current + 1) % max

  private def getLabeledInstr(context: Context, label: String): Int =
    labelMapping.get(label)
      // In case the label is the latest instruction
      .map(instr => instr % operations.length)
      .getOrElse(throw new IllegalArgumentException(
        "Requesting an unknown label: " + label + " not in " + labelMapping.keys))

  private def jumpIf(context: Context, label: String, predicate: Int => Boolean): Context = {
    if (context.node.test(predicate)) {
      val nextInstr = getLabeledInstr(context, label)
      val shiftedNode = context.node.copy(instruction = nextInstr)
      context.copy(node = shiftedNode)
    } else {
      context
    }
  }

}
