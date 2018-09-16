package com.kineolyan.tzio.v1.scala.env

import java.util
import java.util.function.Consumer
import java.util.OptionalInt
import java.util.stream.Stream

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.ops.OperationType
import com.kineolyan.tzio.v1.scala.slot.{EmptySlot, QueueSlot}
import com.kineolyan.tzio.v1.scala.Node
import com.kineolyan.tzio.v1.scala.exec.Execution
import com.kineolyan.tzio.v1.scala.operations.OperationAdapter
import scala.collection.JavaConverters._

class ScalaTzEnv(
                  slots: EnvSlots,
                  nodes: Map[String, Node],
                  executions: Map[String, Execution],
                  consumer: Consumer[Array[OptionalInt]]) extends TzEnv {
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
      executions,
      consumer)
  }

  override def addNode(name: String, memorySize: Int, inputs: Array[Int], outputs: Array[Int], operations: util.List[OperationType]): TzEnv = {
    val node = Node.ofSize(memorySize)
    val newNodes = nodes + (name -> node)

    val ops = operations.asScala
      .map(o => OperationAdapter.convert(o))
      .toArray
    val execution = new Execution(inputs, outputs, ops)
    val newExecs = executions + (name -> execution)

    new ScalaTzEnv(slots, newNodes, newExecs, consumer)
  }

  override def produceInto(consumer: EnvConsumer): TzEnv = new ScalaTzEnv(slots, nodes, executions, consumer)

  override def consume(input: Array[Int]): Unit = {
    // This may be an issue as the call is not chained with anything
    // It is not possible to have an immutable version
    throw new UnsupportedOperationException("Not coded yet")
  }

  override def runFromSystem(args: Array[String]): Unit = {
    throw new UnsupportedOperationException("Not coded yet")
  }

  override def runOn(inputs: Stream[Array[Int]], cycles: Int): Stream[Array[OptionalInt]] = {
    throw new UnsupportedOperationException("Not coded yet")
  }
}

object ScalaTzEnv {
  def empty(): ScalaTzEnv = new ScalaTzEnv(EnvSlots.empty(), Map(), Map(), values => {})
}
