mod mapping;
mod instruction;
mod interface;
mod io;

use parser::ParsingTree;

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

	fn print_report_into<F: FnMut(&str)>(&self, mut out: F) {
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

pub fn check(tree: &ParsingTree) -> CheckResult {
	let mut checks = CheckResult::new();
	// println!("{:?}", res);
	if !mapping::check(tree, &mut checks) {
		checks.add_error(String::from(" -> Mapping errors ..."));
	}
	if !interface::check(tree, &mut checks) {
		checks.add_error(String::from(" -> Node interface errors ..."));
	}
	if !instruction::check(tree, &mut checks) {
		checks.add_error(String::from(" -> Instruction errors ..."));
	}
	if !io::check(tree, &mut checks) {
		checks.add_error(String::from(" -> IOs errors ..."));
	}
	checks
}

#[cfg(test)]
mod tests {
	use super::*;
	use parser::address::{Node, Port};
	use parser::syntax::{InputMapping, OutputMapping};
	use parser::instruction::{Operation, ValuePointer};

	#[test]
	fn test_complete_check_stack() {
    let src = (
      Node::new_node(&"a"),
      vec![
				InputMapping {
					from: Port {
						node: Node::In,
						port: 1
					},
					to: 1
				}
			],
      vec![
        OutputMapping {
          from: 1,
          to: Port {
            node: Node::new_node(&"b"),
            port: 2
          }
        }
      ],
      vec![
				Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))
			]
    );
    let dst = (
      Node::new_node(&"b"),
      vec![
        InputMapping {
          from: Port {
            node: Node::new_node(&"a"),
            port: 1
          },
          to: 2
        }
      ],
      vec![
				OutputMapping {
					from: 2,
					to: Port {
						node: Node::Out,
						port: 3
					}
				}
			],
      vec![
				Operation::MOV(ValuePointer::PORT(2), ValuePointer::PORT(2))
			]
    );
    let tree = vec![src, dst];
    let result = check(&tree);
    assert_eq!(result.has_errors(), false);
	}

	#[test]
	fn test_checker_counts() {
		let mut checks = CheckResult::new();
		assert_eq!(checks.has_errors(), false);
		assert_eq!(checks.has_warnings(), false);
		assert_eq!(checks.error_count(), 0);
		assert_eq!(checks.warning_count(), 0);

		checks.add_error(String::from("e"));
		assert_eq!(checks.has_errors(), true);
		assert_eq!(checks.has_warnings(), false);
		assert_eq!(checks.error_count(), 1);
		assert_eq!(checks.warning_count(), 0);

		checks.add_warning(String::from("w1"));
		checks.add_warning(String::from("w2"));
		assert_eq!(checks.has_errors(), true);
		assert_eq!(checks.has_warnings(), true);
		assert_eq!(checks.error_count(), 1);
		assert_eq!(checks.warning_count(), 2);
	}

	#[test]
	fn test_printing_report() {
		let mut checks = CheckResult::new();
		checks.add_error(String::from("e"));
		checks.add_warning(String::from("w"));
		let mut msgs = vec![];
		checks.print_report_into(|msg| msgs.push(String::from(msg)));

		assert_eq!(msgs, vec![
			" == TZIO compiler == ",
			"1 Warnings in your project",
			"w",
			"1 Errors in your project",
			"e"
		]);
	}

}