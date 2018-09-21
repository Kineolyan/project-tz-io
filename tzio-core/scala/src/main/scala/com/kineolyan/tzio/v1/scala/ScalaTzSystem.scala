package com.kineolyan.tzio.v1.scala

import com.kineolyan.tzio.v1.api.TzEnv
import com.kineolyan.tzio.v1.api.arch.TzSystem
import com.kineolyan.tzio.v1.scala.env.ScalaTzEnv

class ScalaTzSystem extends TzSystem {

  override def createEnv(): TzEnv = {
    ScalaTzEnv.empty()
  }

}
