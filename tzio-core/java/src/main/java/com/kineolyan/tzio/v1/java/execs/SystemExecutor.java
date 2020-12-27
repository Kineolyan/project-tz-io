package com.kineolyan.tzio.v1.java.execs;

import com.kineolyan.tzio.v1.java.JavaTzEnv;
import java.io.InputStream;
import java.io.PrintStream;
import java.util.OptionalInt;
import java.util.concurrent.BlockingDeque;
import java.util.concurrent.LinkedBlockingDeque;
import java.util.concurrent.TimeUnit;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;

/**
 * Implementation of the executor based on streams.
 */
@AllArgsConstructor(access = AccessLevel.PRIVATE)
public class SystemExecutor implements TzExecutor {

	/**
	 * Input stream of this executor
	 */
	private final InputStream in;
	/**
	 * Output stream of this executor
	 */
	private final PrintStream out;

	/**
	 * Creates a new instance from the system.
	 *
	 * @return the created instance
	 */
	public static SystemExecutor fromSystem() {
		return new SystemExecutor(System.in, System.out);
	}

	@Override
	public void run(final JavaTzEnv env) {
		final var outputPrinter = new OutputToStreamCollector(this.out);
		env.produceInto(outputPrinter);

		final BlockingDeque<OptionalInt[]> inputs = new LinkedBlockingDeque<>();
		final BlockingDeque<Throwable> errors = new LinkedBlockingDeque<>();
		final Thread inputThread = createInputThread(inputs, errors);
		inputThread.start();

		try {
			this.out.printf("System up. Waiting for inputs (%d):%n", env.getInputCount());
			while (errors.peek() == null) {
				// Look for entries
				final OptionalInt[] input;
				input = inputs.poll(10, TimeUnit.MILLISECONDS);
				if (input != null) {
					env.consume(input);
				}

				env.tick();
			}
		} catch (InterruptedException e) {
			throw new RuntimeException("Executor interrupted", e);
		}

		final RuntimeException failure = new RuntimeException("Failure while processing inputs");
		errors.forEach(failure::addSuppressed);
		throw failure;
	}

	/**
	 * Creates a thread (not started) parsing the inputs from {@link #in}.
	 *
	 * @param inputs queue filled with the parsed inputs
	 * @param errors errors reported while parsing
	 * @return the thread executing input reading
	 */
	private Thread createInputThread(
			final BlockingDeque<OptionalInt[]> inputs,
			final BlockingDeque<Throwable> errors) {
		final Thread inputThread = new Thread(
				new InputFromStreamCollector(this.in, inputs),
				"input-thread");
		inputThread.setDaemon(true);
		inputThread.setUncaughtExceptionHandler((t, err) -> errors.offer(err));
		return inputThread;
	}
}
