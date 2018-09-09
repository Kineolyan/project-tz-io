package com.kineolyan.tzio.v1.java;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.arch.TzSystem;

public class JavaTzSystem implements TzSystem {

	@Override
	public TzEnv createEnv() {
		return new JavaTzEnv();
	}

}
