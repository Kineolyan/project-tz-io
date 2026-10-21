package com.kineolyan.tzio.v1.multi;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.nio.file.Path;
import java.util.List;
import org.junit.jupiter.api.Test;

class TestKernelConfig {

	private static final Path DIR = Path.of("/tmp");

	@Test
	void testParse() {
		final KernelConfig config = KernelConfig.parse(
				List.of(
						"# A comment",
						"",
						"double => py",
						"main=>rs",
						"py <= python3 kernel.py --flag => x",
						"rs <= ./rs-kernel  "),
				DIR);
		assertThat(config.commandFor("double")).isEqualTo("python3 kernel.py --flag => x");
		assertThat(config.commandFor("main")).isEqualTo("./rs-kernel");
		assertThat(config.directory()).isEqualTo(DIR);
	}

	@Test
	void testUnknownNode() {
		final KernelConfig config = KernelConfig.parse(List.of("py <= python3"), DIR);
		assertThatThrownBy(() -> config.commandFor("other"))
				.hasMessageContaining("other");
	}

	@Test
	void testUnknownKernel() {
		final KernelConfig config = KernelConfig.parse(List.of("node => rs"), DIR);
		assertThatThrownBy(() -> config.commandFor("node"))
				.hasMessageContaining("rs");
	}

	@Test
	void testInvalidLine() {
		assertThatThrownBy(() -> KernelConfig.parse(List.of("node py"), DIR))
				.hasMessageContaining("line 1");
	}

	@Test
	void testDuplicatedNode() {
		assertThatThrownBy(() -> KernelConfig.parse(List.of("a => py", "a => rs"), DIR))
				.hasMessageContaining("line 2");
	}

}
