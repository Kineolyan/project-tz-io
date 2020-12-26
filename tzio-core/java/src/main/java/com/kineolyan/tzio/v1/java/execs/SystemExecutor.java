package com.kineolyan.tzio.v1.java.execs;

import com.kineolyan.tzio.v1.java.JavaTzEnv;

import java.io.InputStream;
import java.io.PrintStream;
import java.util.Scanner;
import java.util.concurrent.BlockingDeque;
import java.util.concurrent.LinkedBlockingDeque;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * Implementation of the executor based on streams.
 */
public class SystemExecutor implements TzExecutor {

	/** Input and output value separator */
	private static final String SPLIT_CHAR = ";";

	/** Input stream of this executor */
	private final InputStream in;
	/** Output stream of this executor */
	private final PrintStream out;

	/**
	 * Constructor
	 * @param in input stream
	 * @param out output stream
	 */
	private SystemExecutor(final InputStream in, final PrintStream out) {
		this.in = in;
		this.out = out;
	}

	/**
	 * Creates a new instance from the system.
	 * @return the created instance
	 */
	public static SystemExecutor fromSystem() {
		return new SystemExecutor(System.in, System.out);
	}

	@Override
	public void run(final JavaTzEnv env) {
		env.produceInto(outputs -> this.out.println(Stream.of(outputs)
			.map(o -> o.isPresent() ? String.valueOf(o.getAsInt()) : "")
			.collect(Collectors.joining(SPLIT_CHAR, "> ", ""))));

		final BlockingDeque<int[]> inputs = new LinkedBlockingDeque<>();
		final BlockingDeque<Throwable> errors = new LinkedBlockingDeque<>();
		final Thread inputThread = new Thread(
			() -> {
				final Scanner scanner = new Scanner(this.in);
				while (scanner.hasNextLine()) {
					final int[] input = Stream.of(scanner.nextLine().split("\\s*" + SPLIT_CHAR + "\\s*"))
						.mapToInt(Integer::parseInt)
						.toArray();
					inputs.offer(input);
				}
			},
			"input-thread");
		inputThread.setDaemon(true);
		inputThread.setUncaughtExceptionHandler((t, err) -> errors.offer(err));
		inputThread.start();

		this.out.printf("System up. Waiting for inputs (%d):%n", env.getInputCount());
		while (errors.peek() == null) {
			// Look for entries
			final int[] input = inputs.poll();
			if (input != null) {
				env.consume(input);
			}

			env.tick();
		}

		final RuntimeException failure = new RuntimeException("Failure while processing inputs");
		errors.forEach(failure::addSuppressed);
		throw failure;
	}
}
