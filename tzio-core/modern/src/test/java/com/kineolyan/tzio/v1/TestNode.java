package com.kineolyan.tzio.v1;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ref.*;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestNode {

	@Test
	void testAccChanges() {
		final Node node = defaultNode();
		assertThat(node.getAccValue()).isEqualTo(0);

		node.setAccValue(10);
		assertThat(node.getAccValue()).isEqualTo(10);

		node.setAccValue(-7);
		assertThat(node.getAccValue()).isEqualTo(-7);
	}

	@Test
	void testMoveValue() {
		final Node node = defaultNode();
		final InputReference input = References.value(112);
		final DataReference output = new DataReference();
		node.moveValue(input, output);

		assertThat(output.getValue().getAsInt()).isEqualTo(112);
	}

	@Test
	void testAddValue() {
		final Node node = defaultNode();
		final InputReference input = References.value(13);

		node.setAccValue(11);
		node.addValue(input);
		assertThat(node.getAccValue()).isEqualTo(24);
	}

	@Test
	void testSubValue() {
		final Node node = defaultNode();
		final InputReference input = References.value(51);

		node.setAccValue(10);
		node.subValue(input);
		assertThat(node.getAccValue()).isEqualTo(-41);
	}

	@Test
	void testNegateValue() {
		final Node node = defaultNode();
		node.setAccValue(42);
		node.negate();
		assertThat(node.getAccValue()).isEqualTo(-42);
	}

	@Test
	void testBakValue() {
		final Node node = defaultNode();
		assertThat(node.getMemoryValue(1)).isEqualTo(0);
		assertThat(node.getMemoryValue(2)).isEqualTo(0);

		node.setAccValue(5);
		node.bakValue(2);
		assertThat(node.getMemoryValue(1)).isEqualTo(0);
		assertThat(node.getMemoryValue(2)).isEqualTo(5);
	}

	@Test
	void testSwapValue() {
		final Node node = defaultNode();
		node.setAccValue(1);
		node.bakValue(1);
		node.setAccValue(2);
		node.bakValue(2);
		assertThat(node.getMemoryValue(1)).isEqualTo(1);
		assertThat(node.getMemoryValue(2)).isEqualTo(2);

		node.setAccValue(3);
		node.swapValue(1);
		assertThat(node.getAccValue()).isEqualTo(1);
		assertThat(node.getMemoryValue(1)).isEqualTo(3);
		assertThat(node.getMemoryValue(2)).isEqualTo(2);
	}

	@Test
	void testNodeValue() {
		final Node node = defaultNode();
		node.setAccValue(4);
		assertThat(node.testValue(v -> v % 4 == 0)).isTrue();
		assertThat(node.testValue(v -> v == 0)).isFalse();
	}

	private Node defaultNode() {
		return new Node(2, new InputSlot[1], new OutputSlot[1]);
	}

}
