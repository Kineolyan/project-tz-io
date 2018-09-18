package com.kineolyan.tzio.v1.scala.env

import java.util
import java.util.function.Consumer
import java.util.{OptionalInt, Spliterators, Spliterator}
import java.util.stream.{Stream, StreamSupport}

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.ops.OperationType
import com.kineolyan.tzio.v1.scala.slot.{EmptySlot, QueueSlot}
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

  def consume(input: Array[Int]): ScalaTzEnv = {
    assert(input.length == slots.inputs.length)

    val filledInputs = slots.inputs.toStream
      .zipWithIndex
      .map(entry => {
        val (inputIdx, orderIdx) = entry
        val slot = slots.slots.apply(inputIdx)
        if (slot isInstanceOf QueueSlot) {
          val value = input.apply(orderIdx)
          (slot asInstanceOf QueueSlot).consume(value)
        } else {
          throw new IllegalStateException("Expecting slot " + slot + " to be a queue slot")
        }
      })
      .toArray
    val newInputs = Stream(0, slots.slots.length)
      

  }

  def collect(): (ScalaTzEnv, Array[OptionalInt]) = {
    throw new UnsupportedOperationException("Not coded yet")
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
