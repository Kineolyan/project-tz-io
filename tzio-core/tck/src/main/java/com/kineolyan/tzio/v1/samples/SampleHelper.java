package com.kineolyan.tzio.v1.samples;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.OptionalInt;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * Helper class to run tests on TZ IO environments.
 */
public class SampleHelper {

	private SampleHelper() {}

	/**
	 * Collects outputs from results.
	 * <p>
	 *   Results are collected by batch. If a cycle first outputs a on OUT1 and the next cycle
	 *   produces b on OUT2, the collected results will be {@code [[a, b]]}.
	 * </p>
	 * @param results produced result stream
	 * @return the collect results
	 * @see #strictCollect(Stream) for strict collections
	 */
	public static List<List<Integer>> batchCollect(Stream<OptionalInt[]> results) {
		final Map<Integer, List<Integer>> outputs = results.sequential()
				.reduce(
						new HashMap<>(),
						(acc, values) -> {
							for (int i = 0; i < values.length; i += 1) {
								final OptionalInt value = values[i];
								if (value.isPresent()) {
									final List<Integer> l = acc.computeIfAbsent(i, __ -> new ArrayList<>());
									l.add(value.getAsInt());
								}
							}
							return acc;
						},
						(a, b) -> { throw new RuntimeException("Should not merge"); });
		final int max = outputs.keySet().stream().max(Integer::compareTo).get() + 1;

		final Map<Integer, Iterator<Integer>> cursors = outputs.entrySet().stream()
				.collect(Collectors.toMap(
						e -> e.getKey(),
						e -> e.getValue().iterator()));
		final List<List<Integer>> batches = new ArrayList<>();
		while (!cursors.isEmpty()) {
			final List<Integer> values = IntStream.range(0, max)
					.mapToObj(i -> {
						final Iterator<Integer> cursor = cursors.get(i);
						if (cursor != null) {
							final Integer val = cursor.next();
							if (!cursor.hasNext()) {
								cursors.remove(i);
							}
							return val;
						} else {
							return null;
						}
					})
					.collect(Collectors.toList());
			batches.add(values);
		}

		return batches;
	}

	/**
	 * Collects outputs from results.
	 * <p>
	 *   Results are collected by iteration. If a cycle first outputs a on OUT1 and the next cycle
	 *   produces b on OUT2, the collected results will be {@code [[a, null], [null, b]]}.
	 * </p>
	 * @param results produced result stream
	 * @return the collect results
	 * @see #batchCollect(Stream) to logically group outputs by batch
	 */
	public static List<List<Integer>> strictCollect(Stream<OptionalInt[]> results) {
		return results
				.map(values -> Stream.of(values)
						.map(value -> value.isPresent() ? value.getAsInt() : null)
						.collect(Collectors.toList()))
				.collect(Collectors.toList());
	}
}
