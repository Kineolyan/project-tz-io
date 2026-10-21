package com.kineolyan.tzio.v1.multi;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Configuration of the sub-kernels running the nodes.
 * <p>
 *   The configuration file contains two kinds of lines:
 *   <ul>
 *     <li>{@code <node name> => <kernel id>}, selecting the kernel running a node</li>
 *     <li>{@code <kernel id> <= <command>}, defining the command starting a kernel</li>
 *   </ul>
 *   Lines starting with {@code #} and blank lines are ignored.
 * </p>
 * @param nodeKernels kernel ids indexed by node names
 * @param kernelCommands kernel commands indexed by kernel ids
 * @param directory directory from which commands are run
 */
public record KernelConfig(
		Map<String, String> nodeKernels,
		Map<String, String> kernelCommands,
		Path directory) {

	/** System property holding the path to the configuration file */
	public static final String CONFIG_PROPERTY = "tzio.multi.config";
	/** Environment variable holding the path to the configuration file */
	public static final String CONFIG_VARIABLE = "TZIO_MULTI_CONFIG";

	private static final Pattern LINE = Pattern.compile("^(\\S+)\\s*(=>|<=)\\s*(.*\\S)\\s*$");

	/**
	 * Loads the configuration from the file defined by the system property, or the environment
	 * variable.
	 * @return the loaded configuration
	 */
	public static KernelConfig fromEnvironment() {
		String path = System.getProperty(CONFIG_PROPERTY);
		if (path == null) {
			path = System.getenv(CONFIG_VARIABLE);
		}
		if (path == null) {
			throw new IllegalStateException(
					"No kernel configuration. Set the system property " + CONFIG_PROPERTY
							+ " or the environment variable " + CONFIG_VARIABLE);
		}
		return load(Path.of(path));
	}

	/**
	 * Loads the configuration from a file.
	 * <p>
	 *   Commands are run from the directory containing the file.
	 * </p>
	 * @param file configuration file
	 * @return the loaded configuration
	 */
	public static KernelConfig load(final Path file) {
		try {
			final Path absoluteFile = file.toAbsolutePath();
			return parse(Files.readAllLines(absoluteFile), absoluteFile.getParent());
		} catch (final IOException e) {
			throw new UncheckedIOException("Cannot read kernel configuration " + file, e);
		}
	}

	/**
	 * Parses the lines of a configuration.
	 * @param lines configuration lines
	 * @param directory directory from which commands are run
	 * @return the parsed configuration
	 */
	public static KernelConfig parse(final List<String> lines, final Path directory) {
		final Map<String, String> nodeKernels = new HashMap<>();
		final Map<String, String> kernelCommands = new HashMap<>();
		for (int i = 0; i < lines.size(); i += 1) {
			final String line = lines.get(i).strip();
			if (line.isEmpty() || line.startsWith("#")) {
				continue;
			}

			final Matcher matcher = LINE.matcher(line);
			if (!matcher.matches()) {
				throw new IllegalArgumentException("Invalid kernel configuration at line " + (i + 1) + ": " + line);
			}
			final Map<String, String> target = matcher.group(2).equals("=>") ? nodeKernels : kernelCommands;
			if (target.put(matcher.group(1), matcher.group(3)) != null) {
				throw new IllegalArgumentException("Duplicated definition at line " + (i + 1) + ": " + line);
			}
		}

		return new KernelConfig(Map.copyOf(nodeKernels), Map.copyOf(kernelCommands), directory);
	}

	/**
	 * Gets the command starting the kernel of a node.
	 * @param nodeName name of the node
	 * @return the kernel command
	 */
	public String commandFor(final String nodeName) {
		final String kernel = this.nodeKernels.get(nodeName);
		if (kernel == null) {
			throw new IllegalStateException("No kernel configured for node " + nodeName);
		}
		final String command = this.kernelCommands.get(kernel);
		if (command == null) {
			throw new IllegalStateException("No command for kernel " + kernel + " of node " + nodeName);
		}
		return command;
	}

}
