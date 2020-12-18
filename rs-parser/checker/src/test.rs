use language::syntax::Program;
use language::test::TestCase;
use language::address::{Node, Port};
use language::syntax::{NodeBlock, InputMapping, OutputMapping};
use crate::result::CheckResult;

/// Module checking that the tests are correctly formed.
/// There must be as many values as inputs and outputs
type Counts = (usize, usize);
fn count_ios(nodes: &Vec<NodeBlock>) -> Counts {
  let mut ins = 0;
  let mut outs = 0;
  for node in nodes {
    ins += node.1.iter()
      .filter(|i| match i {
        InputMapping {from: Port {node: Node::In, port: _}, to: _} => true,
        _ => false
      })
      .count();
    outs += node.2.iter()
      .filter(|o| match o {
        OutputMapping {from: _, to: Port {node: Node::Out, port: _}} => true,
        _ => false
      })
      .count();
  }

  (ins, outs)
}

fn check_test(test: &TestCase, result: &mut CheckResult, counts: &Counts) {
  let (ins, outs) = counts;
  if test.ins.len() != *ins {
    result.add_error(format!(
      "Test case {:?} has not the correct number of inputs. Expecting {}",
      test, ins));
  }
  if test.outs.len() != *outs {
    result.add_error(format!(
      "Test case {:?} has not the correct number of outputs. Expecting {}",
      test, outs));
  }
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
      vec![
        InputMapping {
          from: Port {
            node: Node::In,
            port: 1
          },
          to: 2
        }
      ],
      vec![
        OutputMapping {
          from: 1,
          to: Port {
            node: Node::Out,
            port: 2
          }
        }
      ],
      vec![]
    );
    let dst = (
      Node::new_node(&"b"),
      vec![
        InputMapping {
          from: Port {
            node: Node::In,
            port: 1
          },
          to: 2
        }
      ],
      vec![],
      vec![]
    );
    vec![src, dst]
  }

  #[test]
  fn test_check_valid_tests() {
    let mut check_result = CheckResult::new();
    let tests = vec![
      TestCase { ins: vec![1, 2], outs: vec![9]},
      TestCase { ins: vec![3, 4], outs: vec![8]}
    ];
    let result = check(&Program {nodes: create_nodes(), tests: tests}, &mut check_result);
    assert_eq!(result, true);
    assert_eq!(check_result.has_errors(), false);
  }

  #[test]
  fn test_check_missing_inputs() {
    let tests = vec![
      TestCase { ins: vec![1], outs: vec![9]},
      TestCase { ins: vec![1, 2], outs: vec![9]},
      TestCase { ins: vec![], outs: vec![9]}
    ];

    let mut checks = CheckResult::new();

    let result = check(&Program {nodes: create_nodes(), tests: tests}, &mut checks);
    assert_eq!(result, false);
    assert_eq!(checks.has_errors(), true);
    assert_eq!(checks.error_count(), 2);
  }

  #[test]
  fn test_check_too_many_inputs() {
    let tests = vec![
      TestCase { ins: vec![1, 2, 3], outs: vec![9]},
      TestCase { ins: vec![1, 2], outs: vec![9]},
      TestCase { ins: vec![1, 2, 3, 4], outs: vec![9]}
    ];

    let mut checks = CheckResult::new();

    let result = check(&Program {nodes: create_nodes(), tests: tests}, &mut checks);
    assert_eq!(result, false);
    assert_eq!(checks.has_errors(), true);
    assert_eq!(checks.error_count(), 2);
  }

  #[test]
  fn test_check_missing_outputs() {
    let tests = vec![
      TestCase { ins: vec![1, 2], outs: vec![]},
      TestCase { ins: vec![1, 2], outs: vec![9]},
      TestCase { ins: vec![1, 2], outs: vec![]}
    ];

    let mut checks = CheckResult::new();

    let result = check(&Program {nodes: create_nodes(), tests: tests}, &mut checks);
    assert_eq!(result, false);
    assert_eq!(checks.has_errors(), true);
    assert_eq!(checks.error_count(), 2);
  }

  #[test]
  fn test_check_too_many_outputs() {
    let tests = vec![
      TestCase { ins: vec![1, 2], outs: vec![9, 8, 7]},
      TestCase { ins: vec![1, 2], outs: vec![9]},
      TestCase { ins: vec![1, 2], outs: vec![9, 8]},
    ];

    let mut checks = CheckResult::new();

    let result = check(&Program {nodes: create_nodes(), tests: tests}, &mut checks);
    assert_eq!(result, false);
    assert_eq!(checks.has_errors(), true);
    assert_eq!(checks.error_count(), 2);
  }

}
