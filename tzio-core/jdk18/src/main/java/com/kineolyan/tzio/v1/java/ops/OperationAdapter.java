package com.kineolyan.tzio.v1.java.ops;

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
import com.kineolyan.tzio.v1.java.ref.InputAdapter;
import com.kineolyan.tzio.v1.java.ref.OutputAdapter;
import lombok.RequiredArgsConstructor;

/**
 * Adapter converting operation description into their implementation for this core.
 */
@RequiredArgsConstructor
public class OperationAdapter implements OperationVisitor<Operation> {

	/** Adapter to use for inputs */
	private final InputAdapter inputAdapter;
	/** Adapter to use for outputs */
	private final OutputAdapter outputAdapter;

	public Operation convert(OperationType type) {
		return type.accept(this);
	}

	@Override
	public Operation visit(final MovOperation movOperation) {
		return Operations.MOV(
			this.inputAdapter.convert(movOperation.input()),
			this.outputAdapter.convert(movOperation.output()));
	}

	@Override
	public Operation visit(final SavOperation savOperation) {
		return Operations.SAV(savOperation.slot());
	}

	@Override
	public Operation visit(final SwpOperation swpOperation) {
		return Operations.SWP(swpOperation.slot());
	}

	@Override
	public Operation visit(final AddOperation addOperation) {
		return Operations.ADD(
				this.inputAdapter.convert(addOperation.input()));
	}

	@Override
	public Operation visit(final SubOperation subOperation) {
		return Operations.SUB(
				this.inputAdapter.convert(subOperation.input()));
	}

	@Override
	public Operation visit(final NegOperation negOperation) {
		return Operations.NEG();
	}

	@Override
	public Operation visit(final LabelOperation labelOperation) {
		return Operations.LABEL(labelOperation.label());
	}

	@Override
	public Operation visit(final JmpOperation jmpOperation) {
		return Operations.JMP(jmpOperation.label());
	}

	@Override
	public Operation visit(final JezOperation jezOperation) {
		return Operations.JEZ(jezOperation.label());
	}

	@Override
	public Operation visit(final JnzOperation jnzOperation) {
		return Operations.JNZ(jnzOperation.label());
	}

	@Override
	public Operation visit(final JlzOperation jlzOperation) {
		return Operations.JLZ(jlzOperation.label());
	}

	@Override
	public Operation visit(final JgzOperation jgzOperation) {
		return Operations.JGZ(jgzOperation.label());
	}

	@Override
	public Operation visit(final JroOperation jroOperation) {
		return Operations.JRO(
				this.inputAdapter.convert(jroOperation.input()));
	}

}
