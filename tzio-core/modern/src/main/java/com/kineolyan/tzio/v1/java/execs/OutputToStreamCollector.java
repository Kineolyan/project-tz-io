package com.kineolyan.tzio.v1.java.execs;

import java.io.PrintStream;
import java.util.OptionalInt;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import lombok.AllArgsConstructor;

/**
 * Class accepting the output values produced by a TzEnv, and printing it as a single into a
 * dedicated stream.
 */
@AllArgsConstructor
public class OutputToStreamCollector implements Consumer<OptionalInt[]> {

	private final PrintStream out;

	@Override
	public void accept(final OptionalInt[] outputs) {
		final String asString = getOutputAsString(outputs);
		this.out.println(asString);
	}

	private String getOutputAsString(final OptionalInt[] outputs) {
		return Stream.of(outputs)
				.map(o -> o.isPresent() ? String.valueOf(o.getAsInt()) : "")
				.collect(Collectors.joining(InputFromStreamCollector.SPLIT_CHAR, "> ", ""));
	}

}
