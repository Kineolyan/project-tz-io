package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ops.Operations;
import com.kineolyan.tzio.v1.java.slot.InputSlot;
import com.kineolyan.tzio.v1.java.slot.OutputSlot;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestNilReference {

	private Node node;
	private NilReference ref;

	@BeforeEach
	void setUp() {
		this.node = new Node(1, new InputSlot[1], new OutputSlot[1]);
		this.ref = NilReference.INSTANCE;
	}

	@Test
	void testCanAlwaysRead() {
		assertThat(this.ref.canRead(this.node)).isTrue();
	}

	@Test
	void testCanAlwaysWrite() {
		assertThat(this.ref.canWrite(this.node)).isTrue();
	}

	@Test
	void testReadNodeValue() {
		setBakValue(42);
		this.node.setAccValue(12);

		assertThat(this.ref.readValue(this.node)).isEqualTo(0);
	}

	@Test
	void testWriteIntoNodeValue() {
		// Set the node value and BAK, to ensure that the value come from nowhere
		setBakValue(13);
		this.node.setAccValue(47);

		this.ref.writeValue(this.node, 5);

		// Test that the node did not change
		assertThat(this.node.getAccValue()).isEqualTo(47);
		assertThat(getBakValue()).isEqualTo(13);
	}

	private void setBakValue(final int value) {
		this.node.setAccValue(value);
		Operations.SWP(1).execute(this.node);
	}

	private int getBakValue() {
		Operations.SWP(1).execute(this.node);
		return this.node.getAccValue();
	}

}
