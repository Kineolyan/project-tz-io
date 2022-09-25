/*
 * (C) ActiveViam FS 2013-2018
 * ALL RIGHTS RESERVED. This material is the CONFIDENTIAL and PROPRIETARY
 * property of Quartet Financial Systems Limited. Any unauthorized use,
 * reproduction or transfer of this material is strictly prohibited
 */
package com.kineolyan.tzio.v1.java.ops;

import com.kineolyan.tzio.v1.java.Node;
import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.Test;

class TestNegOperation {

	@Test
	void testNegateValue() {
		final Node node = OperationTestUtil.defaultNode();
		node.setAccValue(4);
		Operations.NEG().execute(node);
		Assertions.assertThat(node.getAccValue()).isEqualTo(-4);

		Operations.NEG().execute(node);
		Assertions.assertThat(node.getAccValue()).isEqualTo(4);
	}

	@Test
	void testShiftToNextOperation() {
		final Node node = OperationTestUtil.defaultNode();
		node.setAccValue(4);
		final Operation.Shift shift = Operations.NEG().execute(node);
		OperationTestUtil.assertThat(shift).shiftToNext();
	}

}
