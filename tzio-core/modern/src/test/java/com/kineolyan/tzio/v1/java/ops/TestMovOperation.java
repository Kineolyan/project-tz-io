package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import com.kineolyan.tzio.v1.java.ref.References;
import com.kineolyan.tzio.v1.java.slot.DataSlot;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestMovOperation {

	private Node node;

	@BeforeEach
	void createNode() {
		this.node = OperationTestUtil.defaultNode();
	}

	@Test
	void testSuccessfulMovBetweenIOs() {
		final DataSlot inputSlot = OperationTestUtil.getInput(this.node, 1);
		inputSlot.write(42);
		inputSlot.onStepEnd();
		assertThat(inputSlot.canRead()).isTrue();

		final DataSlot outputSlot = OperationTestUtil.getOutput(this.node, 2);
		assertThat(outputSlot.canWrite()).isTrue();

		final Operation.Shift shift = Operations.MOV(References.inSlot(1), References.outSlot(2))
			.execute(this.node);
		outputSlot.onStepEnd();
		inputSlot.onStepEnd();

		assertThat(shift).isEqualTo(Operation.Shift.NEXT);
		assertThat(inputSlot.canRead()).isFalse();
		assertThat(outputSlot.canWrite()).isFalse();
		assertThat(outputSlot.getValue()).isEqualTo(42);
	}

	@Test
	void testIoMovWithoutInput() {
		final DataSlot inputSlot = OperationTestUtil.getInput(this.node, 1);
		assertThat(inputSlot.canRead()).isFalse();

		final DataSlot outputSlot = OperationTestUtil.getOutput(this.node, 2);
		assertThat(outputSlot.canWrite()).isTrue();

		final Operation.Shift shift = Operations.MOV(References.inSlot(1), References.outSlot(2))
			.execute(this.node);
		outputSlot.onStepEnd();
		inputSlot.onStepEnd();

		assertThat(shift).isEqualTo(Operation.Shift.STAY);
		assertThat(inputSlot.canRead()).isFalse();
		assertThat(outputSlot.canWrite()).isTrue();
	}

	@Test
	void testMovWithFullOutput() {
		final DataSlot inputSlot = OperationTestUtil.getInput(this.node, 1);
		inputSlot.write(42);
		inputSlot.onStepEnd();
		assertThat(inputSlot.canRead()).isTrue();

		final DataSlot outputSlot = OperationTestUtil.getOutput(this.node, 2);
		outputSlot.write(53);
		outputSlot.onStepEnd();
		assertThat(outputSlot.canWrite()).isFalse();

		final Operation.Shift shift = Operations.MOV(References.inSlot(1), References.outSlot(2))
			.execute(this.node);
		outputSlot.onStepEnd();
		inputSlot.onStepEnd();

		assertThat(shift).isEqualTo(Operation.Shift.STAY);
		assertThat(inputSlot.canRead()).isTrue();
		assertThat(outputSlot.canWrite()).isFalse();
		assertThat(outputSlot.getValue()).isEqualTo(53);
	}

	@Test
	void testMovFromValue() {
		final DataSlot outputSlot = OperationTestUtil.getOutput(this.node, 1);
		assertThat(outputSlot.canWrite()).isTrue();

		final Operation.Shift shift = Operations.MOV(
				References.value(53),
				References.outSlot(1))
			.execute(this.node);
		assertThat(shift).isEqualTo(Operation.Shift.NEXT);
		assertThat(outputSlot.getValue()).isEqualTo(53);
	}

	@Test
	void testMovFromNil() {
		this.node.setAccValue(753);

		final Operation.Shift shift = Operations.MOV(
				References.inNil(),
				References.acc())
			.execute(this.node);
		assertThat(shift).isEqualTo(Operation.Shift.NEXT);

		assertThat(this.node.getAccValue()).isEqualTo(0);
	}

	@Test
	void testMovToNil() {
		final DataSlot inputSlot = OperationTestUtil.getInput(this.node, 1);
		inputSlot.write(14);
		inputSlot.onStepEnd();

		final Operation.Shift shift = Operations.MOV(
				References.inSlot(1),
				References.outNil())
			.execute(this.node);
		assertThat(shift).isEqualTo(Operation.Shift.NEXT);
		inputSlot.onStepEnd();

		assertThat(inputSlot.canRead()).isFalse();
	}

}
