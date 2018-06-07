package com.kineolyan.tzio.v1.ops;

import java.util.Objects;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.ref.References;
import com.kineolyan.tzio.v1.slot.DataSlot;
import org.assertj.core.error.ShouldBeInSameDay;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * @author Kineolyan
 */
class TestConditionalOperations {

	private Node node;

	@BeforeEach
	void setup() {
		this.node = OperationTestUtil.defaultNode();
	}

	@TestFactory
	Stream<DynamicTest> operationTestsStream() {
		final String label = "lbl";
		return Stream.of(
				new Config(Operations.JEZ(label), new int[] {0}, new int[] {1, -1}),
				new Config(Operations.JNZ(label), new int[] {1, -1}, new int[] {0}),
				new Config(Operations.JNZ(label), new int[] {1, -1}, new int[] {0}),
				new Config(Operations.JGZ(label), new int[] {1, 2}, new int[] {0, -3}),
				new Config(Operations.JLZ(label), new int[] {-4, -10}, new int[] {0, 8}))
				.flatMap(config -> {
					return Stream.of(
							DynamicTest.dynamicTest(
									"test " + config.operation + " with valid values",
									() -> {
										for (final int value : config.passingValues) {
											this.node.setAccValue(value);
											final Operation.Shift shift = config.operation.execute(this.node);
											assertThat(shift).as("Shift on " + value).isNotEqualTo(Operation.Shift.NEXT);

											final int nextOpIdx = shift.update(
													l -> label.equals(l) ? 5 : -1,
													0,
													10);
											assertThat(nextOpIdx).isEqualTo(5);
										}
									}),
							DynamicTest.dynamicTest(
									"test " + config.operation + " with other values",
									() -> {
										for (final int value : config.otherValues) {
											this.node.setAccValue(value);
											final Operation.Shift shift = config.operation.execute(this.node);
											assertThat(shift).as("Shift on " + value).isEqualTo(Operation.Shift.NEXT);
										}
									}));
				});
	}

	@Test
	void testJmpOperation() {
		final Operation.Shift shift = Operations.JMP("label").execute(this.node);
		assertThat(shift.update(label -> Objects.equals("label", label) ? 1 : -1, 0, 10)).isEqualTo(1);
		assertThat(shift.update(label -> Objects.equals("label", label) ? 1 : -1, 5, 10)).isEqualTo(1);
		assertThat(shift.update(label -> Objects.equals("label", label) ? 1 : -1, 1, 10)).isEqualTo(1);
	}

	@TestFactory
	Stream<DynamicTest> testJroOperationShifts() {
		final int maxIndex = 10;
		return Stream.of(
				new int[] {0, 0, 0},
				new int[] {0, 5, 5},
				new int[] {-2, 8, 6},
				new int[] {-5, 4, 9},
				new int[] {3, 4, 7},
				new int[] {5, 5, 0}
		).map(config -> DynamicTest.dynamicTest(
				String.format("test JRO on %d (%d -> %d)", config[0], config[1], config[2]),
				() -> {
					final Operation.Shift shift = Operations.JRO(References.value(config[0])).execute(this.node);
					final int nextOpIdx = shift.update(_l -> -1, config[1], maxIndex);
					assertThat(nextOpIdx).isEqualTo(config[2]);
				}
		));
	}

	@Test
	void testJroOperationFromAcc() {
		this.node.setAccValue(13);
		final Operation.Shift shift = Operations.JRO(References.acc()).execute(this.node);
		final int nextOpIdx = shift.update(_l -> -1, 2, 100);
		assertThat(nextOpIdx).isEqualTo(15);
	}

	@Test
	void testJroOperationFromValue() {
		final Operation.Shift shift = Operations.JRO(References.value(5)).execute(this.node);
		final int nextOpIdx = shift.update(_l -> -1, 4, 10);
		assertThat(nextOpIdx).isEqualTo(9);
	}

	@Test
	void testJroOperationFromNil() {
		final Operation.Shift shift = Operations.JRO(References.inNil()).execute(this.node);
		assertThat(shift).isEqualTo(Operation.Shift.STAY);
	}

	@Test
	void testJroOperationFromInput() {
		final DataSlot inputSlot = OperationTestUtil.getInput(this.node, 2);
		inputSlot.write(42);
		inputSlot.onStepEnd();
		assertThat(inputSlot.canRead()).isTrue();

		final Operation.Shift shift = Operations.JRO(References.inSlot(2)).execute(this.node);
		final int nextOpIdx = shift.update(_l -> -1, 4, 10);
		assertThat(nextOpIdx).isEqualTo(6);
	}

	private static class Config {
		public final Operation operation;
		public final int[] passingValues;
		public final int[] otherValues;

		private Config(Operation operation, int[] passingValues, int[] otherValues) {
			this.operation = operation;
			this.passingValues = passingValues;
			this.otherValues = otherValues;
		}
	}

}
