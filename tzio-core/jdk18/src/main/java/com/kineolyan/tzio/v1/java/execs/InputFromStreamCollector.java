package com.kineolyan.tzio.v1.java.execs;

import java.io.InputStream;
import java.util.OptionalInt;
import java.util.Queue;
import java.util.Scanner;
import java.util.stream.Stream;
import lombok.AllArgsConstructor;
import lombok.Cleanup;

/**
 * Class collecting data from a provided input stream.
 *
 * <p>This accepts data in the form {@code "1,2,,4"}, with gaps provided as empty strings.</p>
 */
@AllArgsConstructor
class InputFromStreamCollector implements Runnable {

	/** Input and output value separator */
	static final String SPLIT_CHAR = ";";

	private final InputStream in;
	/** Queue receiving the inputs parsed from {@code #in}. */
	private final Queue<OptionalInt[]> queue;

	@Override
	public void run() {
		@Cleanup final Scanner scanner = new Scanner(this.in);
		while (scanner.hasNextLine()) {
			final OptionalInt[] input = lineToInputs(scanner.nextLine());
			this.queue.offer(input);
		}
	}

	private OptionalInt[] lineToInputs(final String line) {
		return Stream.of(line.split("\\s*" + SPLIT_CHAR + "\\s*"))
				.map(this::inputToValue)
				.toArray(OptionalInt[]::new);
	}

	private OptionalInt inputToValue(final String value) {
		return value.isBlank() ? OptionalInt.empty() : OptionalInt.of(Integer.parseInt(value));
	}

}
