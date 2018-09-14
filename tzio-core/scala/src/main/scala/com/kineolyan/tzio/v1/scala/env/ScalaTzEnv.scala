package com.kineolyan.tzio.v1.scala.env

import java.util
import java.util.function.Consumer
import java.util.{OptionalInt, stream}

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.ops.OperationType
import com.kineolyan.tzio.v1.scala.slot.{EmptySlot, QueueSlot}
import com.kineolyan.tzio.v1.scala.{Execution, Node}

object noopConsumer extends Consumer[Array[OptionalInt]] {
  override def accept(t: Array[OptionalInt]): Unit = {} // Do nothing
}

class ScalaTzEnv(
                  slots: EnvSlots,
                  nodes: Map[String, Node],
                  executions: Map[String, Execution],
                  consumer: Consumer[Array[OptionalInt]] = noopConsumer) extends TzEnv {
  type EnvConsumer = Consumer[Array[OptionalInt]]

  override def withSlots(slotCount: Int, inputs: Array[Int], outputs: Array[Int]): TzEnv = {
    val slots: Array[Any] = Range(0, slotCount)
      .map(i => if (inputs.contains(i)) new QueueSlot(List()) else new EmptySlot)
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

    val execution = null;
    val newExecs = executions + (name -> execution)

    new ScalaTzEnv(slots, newNodes, newExecs, consumer)
  }

  override def produceInto(consumer: EnvConsumer): TzEnv = new ScalaTzEnv(slots, nodes, executions, consumer)

  override def consume(input: Array[Int]): Unit = ???

  override def runFromSystem(args: Array[String]): Unit = ???

  override def runOn(inputs: stream.Stream[Array[Int]], cycles: Int): stream.Stream[Array[OptionalInt]] = ???
}

object ScalaTzEnv {
  def empty(): ScalaTzEnv = new ScalaTzEnv(Array(), Array(), Array(), Map(), Map())
}
