package com.kineolyan.tzio.v1.multi;

import com.kineolyan.tzio.v1.api.ops.AddOperation;
import com.kineolyan.tzio.v1.api.ops.JezOperation;
import com.kineolyan.tzio.v1.api.ops.JgzOperation;
import com.kineolyan.tzio.v1.api.ops.JlzOperation;
import com.kineolyan.tzio.v1.api.ops.JmpOperation;
import com.kineolyan.tzio.v1.api.ops.JnzOperation;
import com.kineolyan.tzio.v1.api.ops.JroOperation;
import com.kineolyan.tzio.v1.api.ops.LabelOperation;
import com.kineolyan.tzio.v1.api.ops.MovOperation;
import com.kineolyan.tzio.v1.api.ops.NegOperation;
import com.kineolyan.tzio.v1.api.ops.OperationType;
import com.kineolyan.tzio.v1.api.ops.OperationVisitor;
import com.kineolyan.tzio.v1.api.ops.SavOperation;
import com.kineolyan.tzio.v1.api.ops.SubOperation;
import com.kineolyan.tzio.v1.api.ops.SwpOperation;
import com.kineolyan.tzio.v1.api.ref.AccReference;
import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.InputReferenceVisitor;
import com.kineolyan.tzio.v1.api.ref.NilReference;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceVisitor;
import com.kineolyan.tzio.v1.api.ref.SlotReference;
import com.kineolyan.tzio.v1.api.ref.ValueReference;
import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Encoder of node programs into the bytecode sent to sub-kernels.
 * <p>
 *   See PROTOCOL.md for the description of the format.
 * </p>
 */
public final class BytecodeEncoder {

	/** Version of the produced bytecode */
	public static final int VERSION = 1;

	public static final int MOV = 0x01;
	public static final int ADD = 0x02;
	public static final int SUB = 0x03;
	public static final int NEG = 0x04;
	public static final int SAV = 0x05;
	public static final int SWP = 0x06;
	public static final int JMP = 0x11;
	public static final int JEZ = 0x12;
	public static final int JNZ = 0x13;
	public static final int JLZ = 0x14;
	public static final int JGZ = 0x15;
	public static final int JRO = 0x16;
	/** Flag marking that the source operand is an immediate i32 value */
	public static final int IMMEDIATE = 0x80;

	/** Operand for the accumulator */
	public static final short ACC = -1;
	/** Operand for NIL */
	public static final short NIL = Short.MIN_VALUE;
	/** Maximal index of a slot operand */
	public static final int MAX_SLOT = 1023;

	private BytecodeEncoder() {}

	/**
	 * Encodes the program of a node.
	 * @param inputCount number of node inputs
	 * @param outputCount number of node outputs
	 * @param memorySize size of the node memory
	 * @param operations operations of the node, labels included
	 * @return the bytecode
	 */
	public static byte[] encode(
			final int inputCount,
			final int outputCount,
			final int memorySize,
			final List<OperationType> operations) {
		final Map<String, Integer> labels = resolveLabels(operations);
		final int opCount = operations.size() - countLabels(operations);

		final ByteArrayOutputStream bytes = new ByteArrayOutputStream();
		final DataOutputStream out = new DataOutputStream(bytes);
		try {
			out.writeByte(VERSION);
			writeU16(out, inputCount);
			writeU16(out, outputCount);
			writeU16(out, memorySize);
			writeU16(out, opCount);
			final Writer writer = new Writer(out, labels);
			for (final OperationType operation : operations) {
				operation.accept(writer);
			}
			out.flush();
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
		return bytes.toByteArray();
	}

	/**
	 * Computes the index of the operation targeted by each label.
	 * <p>
	 *   A label at the end of the program targets the first operation, as execution loops.
	 * </p>
	 */
	private static Map<String, Integer> resolveLabels(final List<OperationType> operations) {
		final Map<String, Integer> labels = new HashMap<>();
		int index = 0;
		for (final OperationType operation : operations) {
			if (operation instanceof LabelOperation(final String label)) {
				if (labels.put(label, index) != null) {
					throw new IllegalArgumentException("Duplicated label " + label);
				}
			} else {
				index += 1;
			}
		}
		final int opCount = index;
		labels.replaceAll((label, target) -> target < opCount ? target : 0);
		return labels;
	}

	private static int countLabels(final List<OperationType> operations) {
		return (int) operations.stream().filter(LabelOperation.class::isInstance).count();
	}

	private static void writeU16(final DataOutputStream out, final int value) throws IOException {
		if (value < 0 || value > 0xFFFF) {
			throw new IllegalArgumentException("Value out of u16 range: " + value);
		}
		out.writeShort(value);
	}

	/**
	 * Converts a slot reference, 1-based, to its 0-based operand.
	 */
	private static short slotOperand(final int slot) {
		final int index = slot - 1;
		if (index < 0 || index > MAX_SLOT) {
			throw new IllegalArgumentException("Slot out of range: " + slot);
		}
		return (short) index;
	}

	/**
	 * Visitor writing each operation into the bytecode.
	 */
	private record Writer(DataOutputStream out, Map<String, Integer> labels) implements OperationVisitor<Void> {

		@Override
		public Void visit(final MovOperation operation) {
			final Source source = Source.of(operation.input());
			writeOpCode(MOV, source);
			writeSource(source);
			writeOperand(operation.output().accept(TARGETS));
			return null;
		}

		@Override
		public Void visit(final SavOperation operation) {
			return writeMemoryOp(SAV, operation.slot());
		}

		@Override
		public Void visit(final SwpOperation operation) {
			return writeMemoryOp(SWP, operation.slot());
		}

		@Override
		public Void visit(final AddOperation operation) {
			return writeSourceOp(ADD, operation.input());
		}

		@Override
		public Void visit(final SubOperation operation) {
			return writeSourceOp(SUB, operation.input());
		}

		@Override
		public Void visit(final NegOperation operation) {
			writeByte(NEG);
			return null;
		}

		@Override
		public Void visit(final LabelOperation operation) {
			// Labels are resolved into jump targets
			return null;
		}

		@Override
		public Void visit(final JmpOperation operation) {
			return writeJump(JMP, operation.label());
		}

		@Override
		public Void visit(final JezOperation operation) {
			return writeJump(JEZ, operation.label());
		}

		@Override
		public Void visit(final JnzOperation operation) {
			return writeJump(JNZ, operation.label());
		}

		@Override
		public Void visit(final JlzOperation operation) {
			return writeJump(JLZ, operation.label());
		}

		@Override
		public Void visit(final JgzOperation operation) {
			return writeJump(JGZ, operation.label());
		}

		@Override
		public Void visit(final JroOperation operation) {
			return writeSourceOp(JRO, operation.input());
		}

		private Void writeSourceOp(final int opCode, final InputReferenceType input) {
			final Source source = Source.of(input);
			writeOpCode(opCode, source);
			writeSource(source);
			return null;
		}

		private Void writeMemoryOp(final int opCode, final int slot) {
			writeByte(opCode);
			// Memory slots are 1-based in the language
			writeU16(slot - 1);
			return null;
		}

		private Void writeJump(final int opCode, final String label) {
			final Integer target = this.labels.get(label);
			if (target == null) {
				throw new IllegalArgumentException("No operation named " + label);
			}
			writeByte(opCode);
			writeU16(target);
			return null;
		}

		private void writeOpCode(final int opCode, final Source source) {
			writeByte(source.immediate() ? opCode | IMMEDIATE : opCode);
		}

		private void writeSource(final Source source) {
			try {
				if (source.immediate()) {
					this.out.writeInt(source.value());
				} else {
					this.out.writeShort(source.value());
				}
			} catch (final IOException e) {
				throw new UncheckedIOException(e);
			}
		}

		private void writeOperand(final short operand) {
			try {
				this.out.writeShort(operand);
			} catch (final IOException e) {
				throw new UncheckedIOException(e);
			}
		}

		private void writeByte(final int value) {
			try {
				this.out.writeByte(value);
			} catch (final IOException e) {
				throw new UncheckedIOException(e);
			}
		}

		private void writeU16(final int value) {
			try {
				BytecodeEncoder.writeU16(this.out, value);
			} catch (final IOException e) {
				throw new UncheckedIOException(e);
			}
		}
	}

	/**
	 * Source operand of an operation.
	 * @param immediate true if the value is an immediate value, false for a reference operand
	 * @param value immediate value or reference operand
	 */
	private record Source(boolean immediate, int value) {

		static Source of(final InputReferenceType input) {
			return input.accept(SOURCES);
		}

	}

	private static final InputReferenceVisitor<Source> SOURCES = new InputReferenceVisitor<>() {

		@Override
		public Source visit(final SlotReference ref) {
			return new Source(false, slotOperand(ref.slot()));
		}

		@Override
		public Source visit(final AccReference ref) {
			return new Source(false, ACC);
		}

		@Override
		public Source visit(final ValueReference ref) {
			return new Source(true, ref.value());
		}

		@Override
		public Source visit(final NilReference ref) {
			return new Source(false, NIL);
		}

	};

	private static final OutputReferenceVisitor<Short> TARGETS = new OutputReferenceVisitor<>() {

		@Override
		public Short visit(final SlotReference ref) {
			return slotOperand(ref.slot());
		}

		@Override
		public Short visit(final AccReference ref) {
			return ACC;
		}

		@Override
		public Short visit(final NilReference ref) {
			return NIL;
		}

	};

}
