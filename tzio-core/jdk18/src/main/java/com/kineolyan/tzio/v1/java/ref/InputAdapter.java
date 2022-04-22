package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.api.ref.AccReference;
import com.kineolyan.tzio.v1.api.ref.InputReferenceType;
import com.kineolyan.tzio.v1.api.ref.InputReferenceVisitor;
import com.kineolyan.tzio.v1.api.ref.NilReference;
import com.kineolyan.tzio.v1.api.ref.SlotReference;
import com.kineolyan.tzio.v1.api.ref.ValueReference;

/**
 * Adapter converting input definitions into implementations for this core.
 */
public class InputAdapter implements InputReferenceVisitor<InputReference> {

	/**
	 * Converts the definition of an input into an actual input.
	 * @param type definition to convert
	 * @return created input
	 */
	public InputReference convert(final InputReferenceType type) {
		return type.accept(this);
	}

	@Override
	public InputReference visit(final SlotReference ref) {
		return References.inSlot(ref.slot);
	}

	@Override
	public InputReference visit(final AccReference ref) {
		return References.acc();
	}

	@Override
	public InputReference visit(final ValueReference ref) {
		return References.value(ref.value);
	}

	@Override
	public InputReference visit(final NilReference ref) {
		return References.inNil();
	}

}
