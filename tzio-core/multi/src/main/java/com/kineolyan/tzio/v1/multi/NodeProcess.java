package com.kineolyan.tzio.v1.multi;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Queue;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

/**
 * Sub-kernel process running a single node.
 * <p>
 *   Lines written by the process on its stdout are collected in the background, to be
 *   polled by the host without blocking.
 * </p>
 */
final class NodeProcess implements AutoCloseable {

	/** Name of the node */
	private final String name;
	/** Running process */
	private final Process process;
	/** Writer to the process stdin */
	private final BufferedWriter stdin;
	/** Lines read from the process stdout, not yet polled */
	private final Queue<String> lines;
	/** Lines read from the process stderr, if traced */
	private final List<String> trace;
	/** Flag marking that the process stdout reached its end */
	private volatile boolean ended;
	/** Thread recording stderr, if traced */
	private Thread traceReader;

	private NodeProcess(final String name, final Process process, final boolean traced) {
		this.name = name;
		this.process = process;
		this.stdin = new BufferedWriter(new OutputStreamWriter(process.getOutputStream(), StandardCharsets.UTF_8));
		this.lines = new ConcurrentLinkedQueue<>();
		this.trace = traced ? Collections.synchronizedList(new ArrayList<>()) : List.of();
		this.ended = false;
	}

	/**
	 * Starts the sub-kernel of a node.
	 * @param name name of the node
	 * @param command shell command starting the sub-kernel
	 * @param directory directory from which the command is run
	 * @param traced true to record the process stderr, false to discard it
	 * @param onActivity action run every time the process writes a line on stdout, or ends
	 * @return the started process
	 */
	static NodeProcess start(
			final String name,
			final String command,
			final Path directory,
			final boolean traced,
			final Runnable onActivity) {
		final ProcessBuilder builder = new ProcessBuilder("sh", "-c", command)
				.directory(directory.toFile())
				.redirectError(traced ? ProcessBuilder.Redirect.PIPE : ProcessBuilder.Redirect.DISCARD);
		final Process process;
		try {
			process = builder.start();
		} catch (final IOException e) {
			throw new UncheckedIOException("Cannot start kernel of node " + name + ": " + command, e);
		}

		final NodeProcess node = new NodeProcess(name, process, traced);
		readLines(name + "-stdout", process.getInputStream(), line -> {
			node.lines.offer(line);
			onActivity.run();
		}, () -> {
			node.ended = true;
			onActivity.run();
		});
		if (traced) {
			node.traceReader = readLines(name + "-stderr", process.getErrorStream(), node.trace::add, () -> {});
		}
		return node;
	}

	private static Thread readLines(
			final String threadName,
			final InputStream stream,
			final Consumer<String> onLine,
			final Runnable onEnd) {
		return Thread.ofVirtual().name(threadName).start(() -> {
			try (BufferedReader reader = new BufferedReader(new InputStreamReader(stream, StandardCharsets.UTF_8))) {
				String line;
				while ((line = reader.readLine()) != null) {
					onLine.accept(line);
				}
			} catch (final IOException e) {
				// The stream is closed with the process, considered as its end
			} finally {
				onEnd.run();
			}
		});
	}

	/**
	 * Gets the name of the node run by this process.
	 * @return the node name
	 */
	String name() {
		return this.name;
	}

	/**
	 * Polls the next line written by the process.
	 * @return the line or null if no line is available
	 */
	String poll() {
		return this.lines.poll();
	}

	/**
	 * Tests if the process stopped writing, after all its lines were polled.
	 * @return true if the process ended
	 */
	boolean hasEnded() {
		return this.ended && this.lines.isEmpty();
	}

	/**
	 * Sends a message to the process.
	 * @param message message to send, without line return
	 */
	void send(final String message) {
		try {
			this.stdin.write(message);
			this.stdin.newLine();
			this.stdin.flush();
		} catch (final IOException e) {
			throw new IllegalStateException("Kernel of node " + this.name + " died", e);
		}
	}

	/**
	 * Gets the lines written by the process on stderr.
	 * @return the recorded lines, empty when not traced
	 */
	List<String> trace() {
		synchronized (this.trace) {
			return List.copyOf(this.trace);
		}
	}

	/**
	 * Describes how the process ended.
	 * @return description of the process end
	 */
	String describeEnd() {
		try {
			if (this.process.waitFor(1, TimeUnit.SECONDS)) {
				return "Kernel of node " + this.name + " exited with code " + this.process.exitValue();
			}
		} catch (final InterruptedException e) {
			Thread.currentThread().interrupt();
		}
		return "Kernel of node " + this.name + " closed its output";
	}

	@Override
	public void close() {
		if (this.process.isAlive()) {
			try {
				send("STOP");
				this.stdin.close();
				if (!this.process.waitFor(1, TimeUnit.SECONDS)) {
					this.process.destroyForcibly();
				}
				if (this.traceReader != null) {
					// Collect the last trace lines
					this.traceReader.join(Duration.ofSeconds(1));
				}
			} catch (final IllegalStateException | IOException e) {
				this.process.destroyForcibly();
			} catch (final InterruptedException e) {
				this.process.destroyForcibly();
				Thread.currentThread().interrupt();
			}
		}
	}

	/**
	 * Kills the process without waiting for it.
	 */
	void kill() {
		this.process.destroyForcibly();
	}

}
