use std::collections::HashSet;
use std::ops::FnOnce;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};
use parser::instruction::{ValuePointer, Operation};
use checker::CheckResult;

/// Module checking that the ports referenced by inputs
/// or outputs for duplicated port numbers.

fn check_ports<T, F: Fn(&T) -> u32>(
    result: &mut CheckResult,
    inputs: &Vec<T>, 
    accessor: F) -> HashSet<u32> {
  let mut values = HashSet::new();
  let mut duplicates = HashSet::new();
  for port in inputs.iter() {
    let value = accessor(port);
    if !values.insert(value) {
      duplicates.insert(value);
    }
  }
  duplicates
}

// fn collect_output_ports(outputs: &Vec<OutputMapping>) -> HashSet<u32> {
//   outputs.iter().map(|ref output| output.from).collect::<HashSet<u32>>()
// }

fn dups_to_str(duplicates: HashSet<u32>) -> String {
  duplicates.iter().fold(String::new(), |mut acc, value| {
    acc.push_str(format!("{},", value).as_str());
    acc
  })
}

fn check_node(node: &NodeBlock, result: &mut CheckResult) {
  let input_duplicates = check_ports(result, &node.1, |ref input| input.to);
  if !input_duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated input ports referenced in {}: {}",
      &node.0, dups_to_str(input_duplicates)));
  }

  let output_duplicates = check_ports(result, &node.2, |ref output| output.from);
  if !output_duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated output ports referenced in {}: {}",
      &node.0, dups_to_str(output_duplicates)));
  }
}

pub fn check(tree: &ParsingTree, result: &mut CheckResult) -> bool {
  let initial_count = result.error_count();
  for node in tree {
    check_node(node, result);
  } 

  initial_count == result.error_count()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn fake_input(i: u32) -> InputMapping {
    InputMapping {
      from: Port::new(Node::In, i),
      to: i
    }
  }

  #[test]
  fn test_check_input_duplicates() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![
        fake_input(1),
        fake_input(2),
        fake_input(3)
      ],
      vec![],
      vec![]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![
        fake_input(1),
        fake_input(2),
        fake_input(3),
        fake_input(2),
        fake_input(3)
      ],
      vec![],
      vec![]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.has_errors(), true);
  }

  fn fake_output(i: u32) -> OutputMapping {
    OutputMapping {
      from: i,
      to: Port::new(Node::Out, i)
    }
  }

  #[test]
  fn test_check_output_duplicates() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![],
      vec![
        fake_output(1),
        fake_output(2),
        fake_output(3)
      ],
      vec![]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![],
      vec![
        fake_output(1),
        fake_output(2),
        fake_output(3),
        fake_output(2),
        fake_output(3)
      ],
      vec![]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.has_errors(), true);
  }
}