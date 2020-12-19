mod instruction;
mod interface;
mod io;
mod mapping;
mod result;
mod test;

use crate::result::CheckResult;
use language::syntax::Program;

pub fn check(tree: &Program) -> CheckResult {
    let mut checks = Default::default();
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
    if !test::check(tree, &mut checks) {
        checks.add_error(String::from(" -> Tests errors ..."));
    }
    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::address::{Node, Port};
    use language::instruction::{Operation, ValuePointer};
    use language::syntax::{InputMapping, OutputMapping};

    #[test]
    fn test_complete_check_stack() {
        let src = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 1,
                },
                to: 1,
            }],
            vec![OutputMapping {
                from: 1,
                to: Port {
                    node: Node::new_node(&"b"),
                    port: 2,
                },
            }],
            vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
        );
        let dst = (
            Node::new_node(&"b"),
            vec![InputMapping {
                from: Port {
                    node: Node::new_node(&"a"),
                    port: 1,
                },
                to: 2,
            }],
            vec![OutputMapping {
                from: 2,
                to: Port {
                    node: Node::Out,
                    port: 3,
                },
            }],
            vec![Operation::MOV(ValuePointer::PORT(2), ValuePointer::PORT(2))],
        );
        let tree = vec![src, dst];
        let result = check(&Program {
            nodes: tree,
            tests: vec![],
        });
        assert_eq!(result.has_errors(), false);
    }

    #[test]
    fn test_checker_counts() {
        let mut checks = CheckResult::default();
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
        let mut checks = CheckResult::default();
        checks.add_error(String::from("e"));
        checks.add_warning(String::from("w"));
        let mut msgs = vec![];
        checks.print_report_into(|msg| msgs.push(String::from(msg)));

        assert_eq!(
            msgs,
            vec![
                " == TZIO compiler == ",
                "1 Warnings in your project",
                "w",
                "1 Errors in your project",
                "e"
            ]
        );
    }
}
