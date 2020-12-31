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
        if *input_slot == 0.into() || *input_slot > (input_count as u8).into() {
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
        if *output_slot == 0.into() || *output_slot > (output_count as u8).into() {
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
                    port: 1.into(),
                },
                to: 2.into(),
            }],
            vec![OutputMapping {
                from: 1.into(),
                to: Port {
                    node: Node::Out,
                    port: 2.into(),
                },
            }],
            vec![],
        );
        let dst = (
            Node::new_node(&"b"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 1.into(),
                },
                to: 2.into(),
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
            .input_into(1.into(), vec![1, 3])
            .input_into(2.into(), vec![2, 4])
            .output_from(1.into(), vec![9, 8]);
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
            .input_into(1.into(), vec![1, 3])
            .input_into(2.into(), vec![2, 4, 6, 8, 10])
            .output_from(1.into(), vec![9]);
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
                .input_into(1.into(), vec![1, 2, 3])
                .output_from(1.into(), vec![9]),
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
                .input_into(1.into(), vec![1, 2])
                .input_into(2.into(), vec![3])
                .input_into(3.into(), vec![5, 6])
                .output_from(1.into(), vec![4]),
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
                .input_into(1.into(), vec![1])
                .input_into(2.into(), vec![2]),
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
                .input_into(1.into(), vec![1])
                .input_into(2.into(), vec![2])
                .output_from(1.into(), vec![3])
                .output_from(4.into(), vec![4]),
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
