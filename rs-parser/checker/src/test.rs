use crate::result::CheckResult;
use language::address::{Node, Port};
use language::syntax::Program;
use language::syntax::{InputMapping, NodeBlock, OutputMapping};
use language::test::TestCase;

fn is_reading_in(mapping: &InputMapping) -> bool {
    matches!(mapping, InputMapping {
            from:
                Port {
                    node: Node::In,
                    port: _,
                },
            to: _
    })
}

fn is_writing_out(mapping: &&OutputMapping) -> bool {
    matches!(mapping, OutputMapping {
        from: _,
        to:
            Port {
                node: Node::Out,
                port: _,
            },
    })
}

/// Module checking that the tests are correctly formed.
/// There must be as many values as inputs and outputs
type Counts = (usize, usize);
fn count_ios(nodes: &[NodeBlock]) -> Counts {
    let mut ins = 0;
    let mut outs = 0;
    for node in nodes {
        ins += node.1.iter().filter(|i| is_reading_in(i)).count();
        outs += node.2.iter().filter(|o| is_writing_out(o)).count();
    }

    (ins, outs)
}

fn check_test_inputs(test: &TestCase, result: &mut CheckResult, input_count: usize) {
    for input_slot in test.ins.keys() {
        if *input_slot == 0u32 || *input_slot > input_count as _ {
            result.add_error(format!(
                "Test case {:?} has values for input {} that does not exist",
                test, *input_slot,
            ));
        }
    }

    match test.ins.len().cmp(&input_count) {
        std::cmp::Ordering::Greater => {
            result.add_error(format!(
                "Test case {:?} has too many inputs ({} / {})",
                test,
                test.ins.len(),
                input_count
            ));
        }
        std::cmp::Ordering::Less => {
            result.add_error(format!(
                "Test case {:?} does not have enough inputs ({} / {}).",
                test,
                test.ins.len(),
                input_count
            ));
        }
        _ => {}
    };
}

fn check_test_outputs(test: &TestCase, result: &mut CheckResult, output_count: usize) {
    for output_slot in test.outs.keys() {
        if *output_slot == 0u32 || *output_slot > output_count as _ {
            result.add_error(format!(
                "Test case {:?} has values for output {} that does not exist",
                test, *output_slot,
            ));
        }
    }

    match test.outs.len().cmp(&output_count) {
        std::cmp::Ordering::Greater => {
            result.add_error(format!(
                "Test case {:?} has too many outputs ({} / {})",
                test,
                test.ins.len(),
                output_count
            ));
        }
        std::cmp::Ordering::Less => {
            result.add_error(format!(
                "Test case {:?} does not have enough outputs ({} / {}).",
                test,
                test.ins.len(),
                output_count
            ));
        }
        _ => {}
    }
}

fn check_test(test: &TestCase, result: &mut CheckResult, counts: &Counts) {
    let (ins, outs) = counts;
    check_test_inputs(test, result, *ins);
    check_test_outputs(test, result, *outs);
}

pub fn check(tree: &Program, result: &mut CheckResult) -> bool {
    let counts = count_ios(&tree.nodes);

    let initial_count = result.error_count();
    for test in &tree.tests {
        check_test(&test, result, &counts);
    }

    result.error_count() == initial_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::address::Port;
    use language::syntax::{InputMapping, OutputMapping};

    fn create_nodes() -> Vec<NodeBlock> {
        let src = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 1,
                },
                to: 2,
            }],
            vec![OutputMapping {
                from: 1,
                to: Port {
                    node: Node::Out,
                    port: 2,
                },
            }],
            vec![],
        );
        let dst = (
            Node::new_node(&"b"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 1,
                },
                to: 2,
            }],
            vec![],
            vec![],
        );
        vec![src, dst]
    }

    #[test]
    fn test_check_valid_tests() {
        let mut check_result = Default::default();
        let tests = TestCase::default()
            .inputInto(1, vec![1, 3])
            .inputInto(2, vec![2, 4])
            .outputFrom(1, vec![9, 8]);
        let result = check(
            &Program {
                nodes: create_nodes(),
                tests: Some(tests),
            },
            &mut check_result,
        );
        assert_eq!(result, true);
        assert_eq!(check_result.has_errors(), false);
    }

    #[test]
    fn test_check_test_inputs_and_outputs_of_different_size() {
        let mut check_result = Default::default();
        let tests = TestCase::default()
            .inputInto(1, vec![1, 3])
            .inputInto(2, vec![2, 4, 6, 8, 10])
            .outputFrom(1, vec![9]);
        let result = check(
            &Program {
                nodes: create_nodes(),
                tests: Some(tests),
            },
            &mut check_result,
        );
        assert_eq!(result, true);
        assert_eq!(check_result.has_errors(), false);
    }

    #[test]
    fn test_check_missing_inputs() {
        let tests = Some(
            TestCase::default()
                .inputInto(1, vec![1, 2, 3])
                .outputFrom(1, vec![9]),
        );

        let mut checks = Default::default();

        let result = check(
            &Program {
                nodes: create_nodes(),
                tests,
            },
            &mut checks,
        );
        assert_eq!(result, false);
        assert_eq!(checks.has_errors(), true);
        assert_eq!(checks.error_count(), 1);
    }

    #[test]
    fn test_check_too_many_inputs() {
        let tests = Some(
            TestCase::default()
                .inputInto(1, vec![1, 2])
                .inputInto(2, vec![3])
                .inputInto(3, vec![5, 6])
                .outputFrom(1, vec![4]),
        );

        let mut checks = Default::default();

        let result = check(
            &Program {
                nodes: create_nodes(),
                tests,
            },
            &mut checks,
        );
        assert_eq!(result, false);
        assert_eq!(checks.has_errors(), true);
        assert_eq!(checks.error_count(), 2);
    }

    #[test]
    fn test_check_missing_outputs() {
        let tests = Some(
            TestCase::default()
                .inputInto(1, vec![1])
                .inputInto(2, vec![2]),
        );

        let mut checks = Default::default();

        let result = check(
            &Program {
                nodes: create_nodes(),
                tests,
            },
            &mut checks,
        );
        assert_eq!(result, false);
        assert_eq!(checks.has_errors(), true);
        assert_eq!(checks.error_count(), 1);
    }

    #[test]
    fn test_check_too_many_outputs() {
        let tests = Some(
            TestCase::default()
                .inputInto(1, vec![1])
                .inputInto(2, vec![2])
                .outputFrom(1, vec![3])
                .outputFrom(4, vec![4]),
        );

        let mut checks = Default::default();

        let result = check(
            &Program {
                nodes: create_nodes(),
                tests,
            },
            &mut checks,
        );
        assert_eq!(result, false);
        assert_eq!(checks.has_errors(), true);
        assert_eq!(checks.error_count(), 2);
    }
}
