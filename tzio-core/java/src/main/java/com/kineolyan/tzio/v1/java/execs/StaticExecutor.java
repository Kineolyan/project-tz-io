package com.kineolyan.tzio.v1.java.execs;

import static java.util.stream.Collectors.toUnmodifiableList;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.java.JavaTzEnv;
import java.util.Iterator;
import java.util.List;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

/**
 * Executor from static input.
 * <p>
 *   This is mostly designed for unit tests, expecting a maximal number of cycles to process the input.
 * </p>
 */
public class StaticExecutor implements TzExecutor {

	/** Stream of inputs */
	private final IntStream[] inputs;
	/** Maximal number of cycles of the executor */
	private final int cycles;
	/** Stream of outputs */
	private Stream<OptionalInt[]> result;

	/**
	 * Constructor.
	 * @param inputs input stream
	 * @param cycles maximal number of cycles to run
	 */
	private StaticExecutor(final IntStream[] inputs, final int cycles) {
		this.inputs = inputs;
		this.cycles = cycles;
	}

	/**
	 * Creates an executor from the input, running a maximal number of cycles.
	 * @param input input stream
	 * @param cycles maximal number of cycles to run
	 * @return the created executor
	 */
	public static StaticExecutor on(
			final IntStream[] input,
			final int cycles) {
		return new StaticExecutor(input, cycles);
	}

	@Override
	public void run(final JavaTzEnv env) {
		feedInputs(env);
		this.result = StreamSupport.stream(
				Spliterators.spliteratorUnknownSize(
					new ExecutorIterator(env),
					Spliterator.ORDERED
				),
				false);
	}

	private void feedInputs(final JavaTzEnv env) {
		final var inputIterators = Stream.of(this.inputs)
				.map(IntStream::iterator)
				.collect(toUnmodifiableList());
		while (inputIterators.stream().anyMatch(Iterator::hasNext)) {
			final var inputValues = collectNextInputs(inputIterators);
			env.consume(inputValues);
		}
	}

	private OptionalInt[] collectNextInputs(final List<PrimitiveIterator.OfInt> inputIterators) {
		return inputIterators.stream()
				.map(it -> it.hasNext() ? OptionalInt.of(it.next()) : OptionalInt.empty())
				.toArray(OptionalInt[]::new);
	}

	/**
	 * Gets the produced result stream.
	 * @return the stream of results
	 */
	public Stream<OptionalInt[]> getResult() {
		return this.result;
	}

	/**
	 * Implementation of an iterator used as a source for the result stream.
	 * <p>
	 *   Walking the iterator will actually execute the TZ IO program.
	 * </p>
	 */
	private class ExecutorIterator implements Iterator<OptionalInt[]> {
		private final JavaTzEnv env;
		private int remaining;
		private OptionalInt[] next;

		public ExecutorIterator(final JavaTzEnv env) {
			this.env = env;
			this.remaining = StaticExecutor.this.cycles;
			this.env.produceInto(result -> this.next = result);
			moveToNext();
		}

		@Override
		public boolean hasNext() {
			return this.next != null;
		}

		@Override
		public OptionalInt[] next() {
			final OptionalInt[] current = this.next;
			moveToNext();
			return current;
		}

		/**
		 * Executes a computation cycle of the provided Tz Env and records its results.
		 */
		private void moveToNext() {
			this.next = null;
			while (this.remaining > 0 && this.next == null) {
				this.env.tick();
				this.remaining -= 1;
			}
		}
	}
}
