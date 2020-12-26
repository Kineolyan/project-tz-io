package com.kineolyan.tzio.v1.java.execs;

import java.util.Iterator;
import java.util.OptionalInt;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

import com.kineolyan.tzio.v1.java.JavaTzEnv;

/**
 * Executor from static input.
 * <p>
 *   This is mostly designed for unit tests, expecting a maximal number of cycles to process the input.
 * </p>
 */
public class StaticExecutor implements TzExecutor {

	/** Stream of inputs */
	private final Stream<int[]> input;
	/** Maximal number of cycles of the executor */
	private final int cycles;
	/** Stream of outputs */
	private Stream<OptionalInt[]> result;

	/**
	 * Constructor.
	 * @param input input stream
	 * @param cycles maximal number of cycles to run
	 */
	private StaticExecutor(final Stream<int[]> input, final int cycles) {
		this.input = input;
		this.cycles = cycles;
	}

	/**
	 * Creates an executor from the input, running a maximal number of cycles.
	 * @param input input stream
	 * @param cycles maximal number of cycles to run
	 * @return the created executor
	 */
	public static StaticExecutor on(
			final Stream<int[]> input,
			final int cycles) {
		return new StaticExecutor(input, cycles);
	}

	@Override
	public void run(final JavaTzEnv env) {
		this.input.forEach(env::consume);
		this.result = StreamSupport.stream(
				Spliterators.spliteratorUnknownSize(
					new ExecutorIterator(env),
					Spliterator.ORDERED
				),
				false);
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
