package com.kineolyan.tzio.v1.scala.env

import com.kineolyan.tzio.v1.scala.exec.Context

abstract class Mapping

case class Input(name: String, idx: Int) extends Mapping {}

case class Output(name: String, idx: Int) extends Mapping {}

/**
  * Mapping between an environment slots and the slots used by nodes.
  * @param mapping mapping from a node slot and its index in the environment slot list
  */
class ContextMapper(mapping: Map[Mapping, Int]) {

  def copy(mapping: Map[Mapping, Int] = mapping) = new ContextMapper(mapping)

  /**
    * Maps the input of a node to the environment slot
    * @param nodeName node name
    * @param inputIdx 0-based index of the input slot inside the node
    * @param slotIdx 0-based index of a environment slot
    * @return the updated context
    */
  def mapInput(nodeName: String, inputIdx: Int, slotIdx: Int): ContextMapper = {
    copy(mapping = mapping + (Input(nodeName, inputIdx) -> slotIdx))
  }

  /**
    * Maps the output of a node to the environment slot
    * @param nodeName node name
    * @param outputIdx 0-based index of the output slot inside the node
    * @param slotIdx 0-based index of a environment slot
    * @return the updated context
    */
  def mapOutput(nodeName: String, outputIdx: Int, slotIdx: Int): ContextMapper = {
    copy(mapping = mapping + (Output(nodeName, outputIdx) -> slotIdx))
  }

  /**
    * Creates the context for the given node
    * @param nodeName name of a node of the environment
    * @param env environment
    * @return the created context
    */
  def createContext(nodeName: String, env: ScalaTzEnv): Context = {
    val (node, exec) = env.nodes(nodeName)
    val execInputs = exec.inputs.indices
      .map(idx => {
        val slotIdx = mapping(Input(nodeName, idx))
        env.slots.input(slotIdx)
      })
      .toArray
    val execOutputs = exec.outputs.indices
      .map(idx => {
        val slotIdx = mapping(Output(nodeName, idx))
        env.slots.output(slotIdx)
      })
      .toArray
    new Context(node, execInputs, execOutputs)
  }

  /**
    * Gets the updated form of a given slot
    * @param idx 0-based index of a slot environment
    * @param initialSlot initial environment slot at the index
    * @param contexts contexts mapped by node name
    * @return the new version of the slot
    */
  def getUpdated(
                  idx: Int,
                  initialSlot: Any,
                  contexts: Map[String, Context]
                ): Any = {
    val input = mapping
      .find({
        case (Input(_, _), slotIdx) => slotIdx == idx
        case _ => false
      })
      .map({case (Input(name, inIdx), _: Int) => contexts(name).inputs.apply(inIdx)})
    val output = mapping
      .find({
        case (Output(_, _), slotIdx) => slotIdx == idx
        case _ => false
      })
      .map({case (Output(name, outIdx), _: Int) => contexts(name).outputs.apply(outIdx)})

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
