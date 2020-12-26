package com.kineolyan.tzio.v1.scala.runner

import java.util.OptionalInt

import com.kineolyan.tzio.v1.scala.env.ScalaTzEnv

/**
  * Executor af an TZ-IO program based on static input data.
  * @param inputs input values
  * @param cycles maximal number of cycles to execute
  */
class StaticExecutor(inputs: LazyList[Array[Int]], cycles: Int) {

  def run(initialEnv: ScalaTzEnv): LazyList[Array[OptionalInt]] = {
    val filledEnv = inputs.foldLeft(initialEnv)((result, inputs) => result.consume(inputs))
    val initialState: (ScalaTzEnv, Option[Array[OptionalInt]]) = (filledEnv, None)

    LazyList.from(0).take(cycles)
      .scanLeft(initialState)((acc, iteration) => {
        val (env, _) = acc
        env.tick().collect()
      })
      .drop(1) // Drop the first value of the scan
      .map(acc => acc._2)
      .filter(e => e.isDefined)
      .map(data => data.get)
  }

}

object StaticExecutor {

  def on(inputs: LazyList[Array[Int]], cycles: Int): StaticExecutor =
    new StaticExecutor(inputs, cycles)

}
