package com.kineolyan.tzio.v1.java;

/**
 * Interface representing elements with a transactional aspect.
 */
public interface TransactionalElement {

	/**
	 * Notifies this element of the start of the transaction.
	 */
	default void onStepStart() {}

	/**
	 * Notifies this element of the end of the transaction.
	 */
	default void onStepEnd() {}

}
