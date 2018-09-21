package com.kineolyan.tzio.v1.scala.env

import com.kineolyan.tzio.v1.scala.exec.Context

abstract class Mapping

case class Input(name: String, idx: Int) extends Mapping {}

case class Output(name: String, idx: Int) extends Mapping {}

class ContextMapper(mapping: Map[Mapping, Int]) {

  def copy(mapping: Map[Mapping, Int] = mapping) = new ContextMapper(mapping)

  def addInput(nodeName: String, inputIdx: Int, slotIdx: Int): ContextMapper = {
    copy(mapping = mapping + (Input(nodeName, inputIdx) -> slotIdx))
  }

  def addOutput(nodeName: String, inputIdx: Int, slotIdx: Int): ContextMapper = {
    copy(mapping = mapping + (Output(nodeName, inputIdx) -> slotIdx))
  }

  def createContext(nodeName: String, env: ScalaTzEnv): Context = {
    val (node, exec) = env.nodes(nodeName)
    val execInputs = exec.inputs
      .map(idx => {
        val slotIdx = mapping(Input(nodeName, idx))
        env.slots.input(idx)
      })
    val execOutputs = exec.outputs
      .map(idx => {
        val slotIdx = mapping(Output(nodeName, idx))
        env.slots.output(idx)
      })
    new Context(node, execInputs, execOutputs)
  }

  def getUpdated(
                  idx: Int,
                  initialSlot: Any,
                  contexts: Map[String, Context]
                ): Any = {
    val input = mapping
      .find({
        case (Input(_, _), slotIdx) => true
        case _ => false
      })
      .map({case (Input(name, inIdx), _: Int) => contexts(name).inputs.apply(inIdx)})
    val output = mapping
      .find({
        case (Output(_, _), slotIdx) => true
        case _ => false
      })
      .map({case (Output(name, outIdx), _: Int) => contexts(name).outputs.apply(idx)})

    if (input.isDefined && output.isDefined) {
      // Sanity check, only one of them mush have changed
      if (input.get != initialSlot) {
        input.get
      } else if (output.get != initialSlot) {
        output.get
      } else {
        // None of them changed, return the initial slot
        initialSlot
      }
    } else if (input.isDefined) {
      input.get
    } else if (output.isDefined) {
      output.get
    } else {
      // The node is not referenced, do not update it
      initialSlot
    }
  }

}

object ContextMapper {

  def empty(): ContextMapper = new ContextMapper(mapping = Map())

}
