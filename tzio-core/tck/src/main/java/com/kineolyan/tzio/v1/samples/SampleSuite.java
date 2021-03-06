package com.kineolyan.tzio.v1.samples;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

@DisplayName("Samples")
public class SampleSuite {

	@Nested
	protected class Increment extends IncrementSampleTest {}
	@Nested
	protected class Sum extends SumSampleTest {}
	@Nested
	protected class Double extends DoubleSampleTest {}
	@Nested
	protected class Diffs extends DiffsSampleTest {}

}
