package com.kineolyan.tzio.v1.scala.env

import com.kineolyan.tzio.v1.scala.exec.Context

abstract class Mapping
case class Input(name: String, idx: Int) extends Mapping {}
case class Output(name: String, idx: Int) extends Mapping {}

class ContextMapper(toContext: Map[Mapping, Int], fromContext: Map[Mapping, Int]) {

  def addInput(nodeName: String, inputIdx: Int, slotIdx: Int): ContextMapper = {
    null
  }

  def addOutput(nodeName: String, inputIdx: Int, slotIdx: Int): ContextMapper = {
    null
  }

  def createContext(nodeName: String): Context = {
    null
  }

}
