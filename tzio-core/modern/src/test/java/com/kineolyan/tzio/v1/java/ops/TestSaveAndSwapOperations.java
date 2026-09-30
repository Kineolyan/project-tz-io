package com.kineolyan.tzio.v1.java.ops;

import java.util.stream.IntStream;

import com.kineolyan.tzio.v1.java.Node;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * @author Kineolyan
 */
class TestSaveAndSwapOperations {

	private Node node;

	@BeforeEach
	void setup() {
		this.node = OperationTestUtil.defaultNode();
	}

	@Test
	void testSave() {
		this.node.setAccValue(5);
		final Operation.Shift shift = Operations.SAV(5).execute(this.node);
		OperationTestUtil.assertThat(shift).shiftToNext();

		assertThat(this.node.getAccValue()).isEqualTo(5);

		this.node.setAccValue(2);
		Operations.SWP(5).execute(this.node);
		assertThat(this.node.getAccValue()).isEqualTo(5);
	}

	@Test
	void testSwap() {
		IntStream.range(1, 9).forEach(i -> {
			this.node.setAccValue(10 * i);
			Operations.SAV(i).execute(this.node);
		});

		this.node.setAccValue(3);
		final Operation.Shift shift = Operations.SWP(7).execute(this.node);
		assertThat(this.node.getAccValue()).isEqualTo(70);
		OperationTestUtil.assertThat(shift).shiftToNext();

		this.node.setAccValue(42);
		Operations.SWP(7).execute(this.node);
		assertThat(this.node.getAccValue()).isEqualTo(3);
	}

}
