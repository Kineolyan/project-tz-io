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
                  val slots: EnvSlots,
                  val nodes: Map[String, (Node, Execution)],
                  val contextMapper: ContextMapper)
  extends TzEnv {
  type EnvConsumer = Consumer[Array[OptionalInt]]

  def copy(
            slots: EnvSlots = slots,
            nodes: Map[String, (Node, Execution)] = nodes,
            contextMapper: ContextMapper = contextMapper) =
    new ScalaTzEnv(slots, nodes, contextMapper)

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
      contextMapper)
  }

  override def addNode(name: String, memorySize: Int, inputs: Array[Int], outputs: Array[Int], operations: util.List[OperationType]): TzEnv = {
    val node = Node.ofSize(memorySize)

    val ops = operations.asScala
      .map(o => OperationAdapter.convert(o))
      .toArray
    val execution = new Execution(inputs, outputs, ops)
    val newNodes = nodes + (name -> (node, execution))

    val mapperWithInputs = inputs.zipWithIndex
      .foldLeft(contextMapper)({ case (mapper, (inputIdx, slotIdx)) => mapper.addInput(name, inputIdx, slotIdx) })
    val fullMapper = outputs.zipWithIndex
      .foldLeft(mapperWithInputs)({ case (mapper, (outputIdx, slotIdx)) => mapper.addOutput(name, outputIdx, slotIdx) })

    new ScalaTzEnv(slots, newNodes, fullMapper)
  }

  /**
    * Runs one cycle of the environment, executing operations for every node.
    *
    * @return the updated environment after execution
    */
  def tick(): ScalaTzEnv = {
    val contexts = nodes.map({ case (name, _) => (name, contextMapper.createContext(name, this)) })

    val updatedContexts = nodes.keys
      .map(name => {
        val ctx = contexts(name)
        val (_, execution) = nodes(name)
        (name, execution.runCycle(ctx))
      })
      .toMap

    val newSlots = slots.slots.zipWithIndex
      .map({case (s, idx) => contextMapper.getUpdated(idx, s, updatedContexts)})
    val updatedNodes = nodes.keys
      .map(name => {
        val node = updatedContexts(name).node
        val (_, execution) = nodes(name)
        (name, (node, execution))
      })
      .toMap

    copy(
      slots = slots.copy(slots = newSlots),
      nodes = updatedNodes)
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
        val input = slots.input(idx)
        if (input.canRead) {
          val (value, newSlot) = input.read()
          (OptionalInt.of(value), newSlot)
        } else {
          (OptionalInt.empty(), input)
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
  def empty(): ScalaTzEnv = new ScalaTzEnv(EnvSlots.empty(), Map(), ContextMapper.empty())
}
