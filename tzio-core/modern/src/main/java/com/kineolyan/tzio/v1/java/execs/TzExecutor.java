package com.kineolyan.tzio.v1.java.execs;

import com.kineolyan.tzio.v1.java.JavaTzEnv;

/**
 * Interface for classes making a {@link JavaTzEnv} run.
 * <p>
 *   This feeds it with inputs and handles the produced outputs.
 * </p>
 */
public interface TzExecutor {

	/**
	 * Runs a given environment.
	 * @param env environment to run.
	 */
	void run(JavaTzEnv env);

}
