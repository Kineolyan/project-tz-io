package com.kineolyan.tzio.v1.multi;

import com.kineolyan.tzio.v1.api.TzEnv;
import com.kineolyan.tzio.v1.api.arch.TzSystem;

/**
 * TZ-IO system running each node in a separate process.
 */
public class MultiTzSystem implements TzSystem {

	@Override
	public TzEnv createEnv() {
		return new MultiTzEnv(KernelConfig::fromEnvironment);
	}

}
