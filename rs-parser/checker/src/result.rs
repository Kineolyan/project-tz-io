pub struct CheckResult {
	warnings: Vec<String>,
	errors: Vec<String>
}

impl CheckResult {
	pub fn new() -> CheckResult {
		CheckResult { warnings: Vec::new(), errors: Vec::new() }
	}

	pub fn add_error(&mut self, message: String) {
		self.errors.push(message);
	}

	pub fn add_warning(&mut self, message: String) {
		self.warnings.push(message);
	}

	pub fn has_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	pub fn has_warnings(&self) -> bool {
		!self.warnings.is_empty()
	}

	pub fn error_count(&self) -> usize {
		self.errors.len()
	}

	pub fn warning_count(&self) -> usize {
		self.warnings.len()
	}

	pub fn print_report(&self) {
		self.print_report_into(|msg| println!("{}", msg));
	}

	pub(crate) fn print_report_into<F: FnMut(&str)>(&self, mut out: F) {
		out(&" == TZIO compiler == ");
		if self.has_warnings() {
			out(&format!("{} Warnings in your project", self.warning_count()));
			for warning in &self.warnings {
				out(&warning);
			}
		}
		if self.has_errors() {
			out(&format!("{} Errors in your project", self.error_count()));
			for error in &self.errors {
				out(&error);
			}
		}
	}
}
