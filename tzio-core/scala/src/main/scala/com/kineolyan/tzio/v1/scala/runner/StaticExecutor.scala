package com.kineolyan.tzio.v1.scala.runner

import java.util.OptionalInt

import com.kineolyan.tzio.v1.scala.env.ScalaTzEnv

class StaticExecutor(inputs: Stream[Array[Int]], cycles: Int) {

  def run(initialEnv: ScalaTzEnv): Stream[Array[OptionalInt]] = {
    val filledEnv = inputs.foldLeft(initialEnv)((result, inputs) => result.consume(inputs))
    val initialState: (ScalaTzEnv, Array[OptionalInt]) = (filledEnv, Array())
    Stream(0, cycles)
      .scanLeft(initialState)((acc, iteration) => {
        val (env, _) = acc
        env.tick().collect()
      })
      .drop(1) // Drop the first value of the scan
      .map(dbg => {
        println(dbg._1)
        dbg
      })
      .map(acc => acc._2)
  }

}

object StaticExecutor {

  def on(inputs: Stream[Array[Int]], cycles: Int): StaticExecutor =
    new StaticExecutor(inputs, cycles)

}
