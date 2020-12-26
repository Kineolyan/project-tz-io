package com.kineolyan.tzio.v1.scala.runner

import java.io.{InputStream, PrintStream}
import java.util.Scanner
import java.util.concurrent.LinkedBlockingDeque

import com.kineolyan.tzio.v1.scala.env.ScalaTzEnv

/**
 * Implementation of the executor based on streams.
 */
class SystemExecutor(in: InputStream, out: PrintStream) {

	def run(env: ScalaTzEnv): Unit = {
		val inputs = new LinkedBlockingDeque[Array[Int]]()
		val errors = new LinkedBlockingDeque[Throwable]()
		val inputThread = new Thread(
			() => {
				val scanner = new Scanner(in)
				while (scanner.hasNextLine) {
					val input = scanner.nextLine()
						.split("\\s*" + SystemExecutor.SPLIT_CHAR + "\\s*")
						.map(c => c.toInt)
					inputs.offer(input)
				}
			},
			"input-thread")
		inputThread.setDaemon(true)
		inputThread.setUncaughtExceptionHandler((t: Thread, err: Throwable) => errors.offer(err))
		inputThread.start()

		out.println(s"System up. Waiting for inputs (${env.slots.inputs.length}):")

		var changedEnv = env
		while (errors.peek() == null) {
			// Look for entries
			val input = inputs.poll()
			if (input != null) {
				changedEnv = changedEnv.consume(input)
			}

			val envAfterTick = changedEnv.tick()
			val result = envAfterTick.collect()
			changedEnv = result._1
			result._2 match {
				case Some(output) =>
					val values = output.map(v => if (v.isPresent) v.getAsInt.toString else "")
						.mkString(SystemExecutor.SPLIT_CHAR)
					out.print("> ")
					out.println(values)
				case None => // Do nothing
			}
		}

		if (!errors.isEmpty) {
			val failure = new RuntimeException("Failure while processing inputs")
			errors.forEach(e => failure.addSuppressed(e))
			throw failure
		}
	}
}

object SystemExecutor {

	/** Input and output value separator */
	private def SPLIT_CHAR: String = ";"

	/**
		* Creates a new instance from the system.
		* @return the created instance
		*/
	def fromSystem(): SystemExecutor =
		new SystemExecutor(System.in, System.out)

}
