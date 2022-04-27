package com.kineolyan.tzio.v1.java.ref;

import com.kineolyan.tzio.v1.api.ref.AccReference;
import com.kineolyan.tzio.v1.api.ref.NilReference;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceType;
import com.kineolyan.tzio.v1.api.ref.OutputReferenceVisitor;
import com.kineolyan.tzio.v1.api.ref.SlotReference;

public class OutputAdapter implements OutputReferenceVisitor<OutputReference> {

	public OutputReference convert(final OutputReferenceType type) {
		return switch (type) {
			case SlotReference slot -> visit(slot);
			case NilReference nil -> visit(nil);
			case AccReference acc -> visit(acc);
		};
	}

	@Override
	public OutputReference visit(final SlotReference ref) {
		return References.outSlot(ref.slot());
	}

	@Override
	public OutputReference visit(final AccReference ref) {
		return References.acc();
	}

	@Override
	public OutputReference visit(final NilReference ref) {
		return References.outNil();
	}

}
