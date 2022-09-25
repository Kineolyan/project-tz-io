package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;

import java.util.function.ToIntFunction;

/**
 * Operation shifting the stack to another operation.
 *
 * @param targetLabel Target label to go when executing this operation
 */
record JmpOperation(String targetLabel) implements Operation, Operation.Shift {

    @Override
    public Shift execute(final Node node) {
        return this;
    }

    @Override
    public int update(final ToIntFunction<String> labelIndex, final int current, final int max) {
        return labelIndex.applyAsInt(this.targetLabel);
    }
}
