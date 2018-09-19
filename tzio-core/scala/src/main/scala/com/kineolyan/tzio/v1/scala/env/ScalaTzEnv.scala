package com.kineolyan.tzio.v1.scala.env

import java.util
import java.util.function.Consumer
import java.util.{OptionalInt, Spliterator, Spliterators}
import java.util.stream.{Stream, StreamSupport}

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.ops.OperationType
import com.kineolyan.tzio.v1.scala.slot.{EmptySlot, InputSlot, OutputSlot, QueueSlot}
import com.kineolyan.tzio.v1.scala.Node
import com.kineolyan.tzio.v1.scala.exec.Execution
import com.kineolyan.tzio.v1.scala.operations.OperationAdapter
import com.kineolyan.tzio.v1.scala.runner.StaticExecutor

import scala.collection.JavaConverters._

class ScalaTzEnv(
                  slots: EnvSlots,
                  nodes: Map[String, Node],
                  executions: Map[String, Execution])
  extends TzEnv {
  type EnvConsumer = Consumer[Array[OptionalInt]]

  def copy(
            slots: EnvSlots = slots,
            nodes: Map[String, Node] = nodes,
            executions: Map[String, Execution] = executions) =
    new ScalaTzEnv(slots, nodes, executions)

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
    val newNodes = nodes + (name -> node)

    val ops = operations.asScala
      .map(o => OperationAdapter.convert(o))
      .toArray
    val execution = new Execution(inputs, outputs, ops)
    val newExecs = executions + (name -> execution)

    new ScalaTzEnv(slots, newNodes, newExecs)
  }

  /**
    * Runs one cycle of the environment, executing operations for every node.
    * @return the updated environment after execution
    */
  def tick(): ScalaTzEnv = this

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
