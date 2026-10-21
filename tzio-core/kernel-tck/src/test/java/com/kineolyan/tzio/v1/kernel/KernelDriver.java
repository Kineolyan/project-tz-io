package com.kineolyan.tzio.v1.kernel;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.fail;

import com.kineolyan.tzio.v1.api.ops.OperationType;
import com.kineolyan.tzio.v1.multi.BytecodeEncoder;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Base64;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

/**
 * Driver of a sub-kernel running a single node, for tests.
 * <p>
 *   The kernel is started with the command of the system property {@value #COMMAND_PROPERTY}.
 *   Waits are bounded, so that a non-conforming kernel fails the tests instead of blocking them.
 * </p>
 */
final class KernelDriver implements AutoCloseable {

	/** System property holding the command starting the tested kernel */
	static final String COMMAND_PROPERTY = "tzio.kernel.command";
	/** Maximal time to wait for an expected message or for the kernel exit */
	private static final Duration TIMEOUT = Duration.ofSeconds(5);
	/** Time without message for the kernel to be considered silent */
	private static final Duration SILENCE = Duration.ofMillis(200);

	private final Process process;
	private final BufferedWriter stdin;
	private final BlockingQueue<String> stdout;
	private final List<String> stderr;
	private final Thread stderrReader;

	private KernelDriver(final Process process) {
		this.process = process;
		this.stdin = new BufferedWriter(new OutputStreamWriter(process.getOutputStream(), StandardCharsets.UTF_8));
		this.stdout = new LinkedBlockingQueue<>();
		this.stderr = Collections.synchronizedList(new ArrayList<>());
		readLines(process.getInputStream(), this.stdout::add);
		this.stderrReader = readLines(process.getErrorStream(), this.stderr::add);
	}

	/**
	 * Starts the tested kernel.
	 * @return the driver of the started kernel
	 */
	static KernelDriver launch() {
		final String command = System.getProperty(COMMAND_PROPERTY);
		if (command == null) {
			throw new IllegalStateException("No kernel to test. Set the system property " + COMMAND_PROPERTY);
		}
		try {
			return new KernelDriver(new ProcessBuilder("sh", "-c", command).start());
		} catch (final IOException e) {
			throw new UncheckedIOException("Cannot start kernel " + command, e);
		}
	}

	/**
	 * Starts the tested kernel, loads a program and starts its execution.
	 * @param inputs number of node inputs
	 * @param outputs number of node outputs
	 * @param memory size of the node memory
	 * @param operations program of the node
	 * @return the driver of the running node
	 */
	static KernelDriver run(final int inputs, final int outputs, final int memory, final OperationType... operations) {
		final KernelDriver driver = launch();
		driver.load(BytecodeEncoder.encode(inputs, outputs, memory, List.of(operations)));
		driver.start();
		return driver;
	}

	private static Thread readLines(final InputStream stream, final Consumer<String> onLine) {
		return Thread.ofVirtual().start(() -> {
			try (BufferedReader reader = new BufferedReader(new InputStreamReader(stream, StandardCharsets.UTF_8))) {
				String line;
				while ((line = reader.readLine()) != null) {
					onLine.accept(line);
				}
			} catch (final IOException e) {
				// Stream closed with the process
			}
		});
	}

	/**
	 * Sends a raw message to the kernel.
	 * @param message message, without line return
	 * @return this
	 */
	KernelDriver send(final String message) {
		try {
			this.stdin.write(message);
			this.stdin.newLine();
			this.stdin.flush();
		} catch (final IOException e) {
			throw new UncheckedIOException("Cannot write to the kernel", e);
		}
		return this;
	}

	/**
	 * Loads a program, expecting the kernel to be ready.
	 * @param bytecode program of the node
	 * @return this
	 */
	KernelDriver load(final byte[] bytecode) {
		send("LOAD " + Base64.getEncoder().encodeToString(bytecode));
		return expect("READY");
	}

	/**
	 * Starts the execution of the node.
	 * @return this
	 */
	KernelDriver start() {
		return send("START");
	}

	/**
	 * Sends new input values and freed outputs.
	 * @param values values as {@code <input>:<value>,...}, or {@code -}
	 * @param frees outputs as {@code <output>,...}, or {@code -}
	 * @return this
	 */
	KernelDriver sync(final String values, final String frees) {
		return send("SYNC " + values + " " + frees);
	}

	/**
	 * Waits for the next message of the kernel.
	 * @return the message
	 */
	String next() {
		try {
			final String line = this.stdout.poll(TIMEOUT.toMillis(), TimeUnit.MILLISECONDS);
			if (line == null) {
				fail("No message from the kernel after " + TIMEOUT + describeState());
			}
			return line;
		} catch (final InterruptedException e) {
			Thread.currentThread().interrupt();
			throw new IllegalStateException(e);
		}
	}

	/**
	 * Expects the next messages of the kernel.
	 * @param messages expected messages, in order
	 * @return this
	 */
	KernelDriver expect(final String... messages) {
		for (final String message : messages) {
			assertThat(next()).as("Message from the kernel").isEqualTo(message);
		}
		return this;
	}

	/**
	 * Expects the kernel not to send any message for a short time.
	 * @return this
	 */
	KernelDriver expectSilence() {
		try {
			final String line = this.stdout.poll(SILENCE.toMillis(), TimeUnit.MILLISECONDS);
			assertThat(line).as("Unexpected message from the kernel").isNull();
		} catch (final InterruptedException e) {
			Thread.currentThread().interrupt();
			throw new IllegalStateException(e);
		}
		return this;
	}

	/**
	 * Stops the kernel, expecting it to exit successfully.
	 */
	void stop() {
		send("STOP");
		assertThat(awaitExit()).as("Exit code of the kernel").isZero();
	}

	/**
	 * Waits for the kernel to exit.
	 * @return the exit code
	 */
	int awaitExit() {
		try {
			if (!this.process.waitFor(TIMEOUT.toMillis(), TimeUnit.MILLISECONDS)) {
				fail("Kernel still running after " + TIMEOUT);
			}
			this.stderrReader.join(TIMEOUT);
			return this.process.exitValue();
		} catch (final InterruptedException e) {
			Thread.currentThread().interrupt();
			throw new IllegalStateException(e);
		}
	}

	/**
	 * Closes the kernel stdin.
	 */
	void closeInput() {
		try {
			this.stdin.close();
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}

	/**
	 * Gets the lines written by the kernel on stderr.
	 * <p>
	 *   Lines are complete once the kernel exited.
	 * </p>
	 * @return the lines
	 */
	List<String> stderr() {
		synchronized (this.stderr) {
			return List.copyOf(this.stderr);
		}
	}

	private String describeState() {
		final List<String> errors = stderr();
		final String alive = this.process.isAlive() ? "running" : "exited with " + this.process.exitValue();
		return " (kernel " + alive + ", last stderr lines: "
				+ errors.subList(Math.max(0, errors.size() - 5), errors.size()) + ")";
	}

	@Override
	public void close() {
		this.process.destroyForcibly();
	}

}
