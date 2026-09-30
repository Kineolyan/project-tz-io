package com.kineolyan.tzio.v1.java.ops;

import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.Test;

class TestLabelOperation {

	@Test
	void testGettingLabel() {
		final LabelOperation op = new LabelOperation("lbl");
		Assertions.assertThat(op.label()).isEqualTo("lbl");
	}

}
