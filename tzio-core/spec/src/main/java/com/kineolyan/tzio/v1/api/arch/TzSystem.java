package com.kineolyan.tzio.v1.api.arch;

import java.util.Iterator;
import java.util.ServiceLoader;
import java.util.logging.Level;
import java.util.logging.Logger;

import com.kineolyan.tzio.v1.api.TzEnv;

/**
 * Service interface serving as an entry point for each implementation of TZ environments.
 */
public interface TzSystem {

  /**
   * Gets an instance of the loaded service.
   * <p>
   *   This will throw if more than one implementation is provided for the service.
   * </p>
   * @return the created service instance.
   */
  static TzSystem getInstance() {
    final ServiceLoader<TzSystem> loader = ServiceLoader.load(TzSystem.class);
    final Iterator<TzSystem> it = loader.iterator();
    while (it.hasNext()) {
        TzSystem system = it.next();
        if (!it.hasNext()) {
          final Logger logger = Logger.getLogger(TzSystem.class.getName());
          if (logger.isLoggable(Level.FINE)) {
            logger.fine("Selected TZ-IO system: " + system);
          }

          return system;
        } else {
          throw new IllegalStateException("Too many loaders for " + TzSystem.class.getName());
        }
    }
    throw new IllegalStateException("No loader for " + TzSystem.class.getName());
  }

  /**
   * Creates a new environment for TZ-IO
   * @return the created environment
   */
  TzEnv createEnv();

}
