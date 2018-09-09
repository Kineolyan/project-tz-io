package com.kineolyan.tzio.v1.samples;

import java.util.List;
import java.util.OptionalInt;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class SampleHelper {

	private SampleHelper() {}

	public static List<List<Integer>> collect(Stream<OptionalInt[]> results) {
		return results
			.map(values -> Stream.of(values)
				.map(value -> value.isPresent() ? value.getAsInt() : null)
				.collect(Collectors.toList()))
			.collect(Collectors.toList());
	}
}
