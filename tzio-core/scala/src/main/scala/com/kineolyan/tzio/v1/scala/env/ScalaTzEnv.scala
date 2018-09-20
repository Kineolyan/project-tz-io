package com.kineolyan.tzio.v1.scala.env

import java.util
import java.util.function.Consumer
import java.util.stream.{Stream, StreamSupport}
import java.util.{OptionalInt, Spliterator, Spliterators}

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.ops.OperationType
import com.kineolyan.tzio.v1.scala.Node
import com.kineolyan.tzio.v1.scala.exec.{Context, Execution}
import com.kineolyan.tzio.v1.scala.operations.OperationAdapter
import com.kineolyan.tzio.v1.scala.runner.StaticExecutor
import com.kineolyan.tzio.v1.scala.slot.{EmptySlot, InputSlot, OutputSlot, QueueSlot}

import scala.collection.JavaConverters._

class ScalaTzEnv(
                  slots: EnvSlots,
                  nodes: Map[String, (Node, Execution)],
                  contextMapper: ContextMapper)
  extends TzEnv {
  type EnvConsumer = Consumer[Array[OptionalInt]]

  def copy(
            slots: EnvSlots = slots,
            nodes: Map[String, (Node, Execution)] = nodes) =
    new ScalaTzEnv(slots, nodes, )

  override def withSlots(slotCount: Int, inputs: Array[Int], outputs: Array[Int]): TzEnv = {
    val slots: Array[Any] = Range(0, slotCount)
      .map(i => if (inputs.contains(i)) new QueueSlot(List()) else EmptySlot())
      .toArray
    new ScalaTzEnv(
      new EnvSlots(
        slots,
        inputs,
        outputs),
      nodes,
      executions)
  }

  override def addNode(name: String, memorySize: Int, inputs: Array[Int], outputs: Array[Int], operations: util.List[OperationType]): TzEnv = {
    val node = Node.ofSize(memorySize)

    val ops = operations.asScala
      .map(o => OperationAdapter.convert(o))
      .toArray
    val execution = new Execution(inputs, outputs, ops)
    val newNodes = nodes + (name -> (node, execution))

    val mapperWithInputs = inputs.zipWithIndex
      .foldLeft(contextMapper)({case (mapper, (inputIdx, slotIdx) => mapper.addInput(name, inputIdx, slotIdx)})
    val fullMapper = outputs.zipWithIndex
      .foldLeft(mapperWithInputs)({case (mapper, (outputIdx, slotIdx) => mapper.addOutput(name, outputIdx, slotIdx)})

    new ScalaTzEnv(slots, newNodes, fullMapper)
  }

  /**
    * Runs one cycle of the environment, executing operations for every node.
    * @return the updated environment after execution
    */
  def tick(): ScalaTzEnv = {
    val executionOrder = nodes.keys
    val contexts = executionOrder.toStream.map(name => {
      val (node, exec) = nodes(name)
      val execInputs = exec.inputs
        .map(idx => slots.slots.apply(idx))
        .map({
          case s: InputSlot => s
          case slot => throw new IllegalStateException("Expecting " + slot + " to be an input")
        })
      val execOutputs = exec.outputs
        .map(idx => slots.slots.apply(idx))
        .map({
          case s: OutputSlot => s
          case slot => throw new IllegalStateException("Expecting " + slot + " to be an output")
        })
      new Context(node, execInputs, execOutputs)
    })

    val updatedContext = contexts.zip(executionOrder.toStream.map(name => nodes(name)._2))
      .map({case (ctx, execution) => execution.runCycle(ctx)})

    null
  }

  def consume(input: Array[Int]): ScalaTzEnv = {
    assert(input.length == slots.inputs.length)

    val filledSlot$ = slots.inputs.toStream
      .zipWithIndex
      .map(entry => {
        val (inputIdx, orderIdx) = entry
        val slot = slots.slots.apply(inputIdx)
        slot match {
          case queue: QueueSlot =>
            val value = input.apply(orderIdx)
            (inputIdx, queue.consume(value))
          case _ =>
            throw new IllegalStateException("Expecting slot " + slot + " to be a queue slot")
        }
      })
    val updatedSlots = filledSlot$
      .foldLeft(slots.slots.clone())((acc, entry) => {
        val (inputIdx, newInput) = entry
        acc.update(inputIdx, newInput)
        acc
      })

    copy(slots = slots.copy(slots = updatedSlots))
  }

  def collect(): (ScalaTzEnv, Array[OptionalInt]) = {
    val results = slots.outputs
      .map(idx => {
        val slot = slots.slots.apply(idx)
        slot match {
          case input: InputSlot =>
            if (input.canRead) {
              val (value, newSlot) = input.read()
              (OptionalInt.of(value), newSlot)
            } else {
              (OptionalInt.empty(), input)
            }
          case _ =>
            throw new IllegalStateException("Expecting slot " + slot + " to be a readable slot")
        }
      })
    val updatedSlots = (slots.outputs zip results)
      .foldLeft(slots.slots.clone())((acc, entry) => {
        val (idx, (_, slot)) = entry
        acc.update(idx, slot)
        acc
      })
    val newEnv = copy(slots = slots.copy(slots = updatedSlots))

    val output = results.map(entry => entry._1)
    (newEnv, output)
  }

  override def runFromSystem(args: Array[String]): Unit = {
    throw new UnsupportedOperationException("Not coded yet")
  }

  override def runOn(inputs: Stream[Array[Int]], cycles: Int): Stream[Array[OptionalInt]] = {
    val executor = StaticExecutor.on(
      inputs.iterator().asScala.toStream,
      cycles)
    val result = executor.run(this)
    StreamSupport.stream(
      Spliterators.spliteratorUnknownSize(
        result.toIterator.asJava,
        Spliterator.ORDERED),
      false)
  }
}

object ScalaTzEnv {
  def empty(): ScalaTzEnv = new ScalaTzEnv(EnvSlots.empty(), Map(), Map())
}
