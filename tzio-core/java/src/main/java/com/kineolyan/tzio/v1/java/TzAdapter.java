package com.kineolyan.tzio.v1.java;

import com.kineolyan.tzio.v1.api.ops.OperationType;
import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;
import com.kineolyan.tzio.v1.java.ops.Operation;
import com.kineolyan.tzio.v1.java.ops.OperationAdapter;
import com.kineolyan.tzio.v1.java.ref.InputAdapter;
import com.kineolyan.tzio.v1.java.ref.InputReference;
import com.kineolyan.tzio.v1.java.ref.OutputAdapter;
import com.kineolyan.tzio.v1.java.ref.OutputReference;

class TzAdapter {

	private final OperationAdapter operationAdapter;
	private final InputAdapter inputAdapter;
	private final OutputAdapter outputAdapter;

	TzAdapter() {
		this.inputAdapter = new InputAdapter();
		this.outputAdapter = new OutputAdapter();
		this.operationAdapter = new OperationAdapter(
				this.inputAdapter,
				this.outputAdapter);
	}

	Operation convert(OperationType type) {
		return this.operationAdapter.convert(type);
	}

	InputReference convert(InputReferenceType type) {
		return this.inputAdapter.convert(type);
	}

	OutputReference convert(OutputReferenceType type) {
		return this.outputAdapter.convert(type);
	}

}
