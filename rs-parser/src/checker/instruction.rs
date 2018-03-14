use std::collections::HashSet;
use std::ops::FnOnce;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};
use parser::instruction::{ValuePointer, Operation};
use checker::CheckResult;

/// Module checking that the ports referenced by instructions
/// are defined in the inputs/outputs.
/// This only generate warnings.

fn collect_input_ports(inputs: &Vec<InputMapping>) -> HashSet<u32> {
  inputs.iter().map(|ref input| input.to).collect::<HashSet<u32>>()
}

fn collect_output_ports(outputs: &Vec<OutputMapping>) -> HashSet<u32> {
  outputs.iter().map(|ref output| output.from).collect::<HashSet<u32>>()
}

// fn test_pointer<F: FnOnce(u32) -> String>(
//     result: &mut CheckResult, 
//     inputs: &HashSet<u32>,
//     pointer: &ValuePointer,
//     fmt: F) {
//   if let &ValuePointer::PORT(ref port) = pointer {
//     if !inputs.contains(port) {
//       result.add_warning(fmt(port));
//     }
//   }
// }

fn test_input(
    result: &mut CheckResult, 
    inputs: &HashSet<u32>,
    node: &Node,
    op: &Operation, 
    pointer: &ValuePointer) {
  if let &ValuePointer::PORT(ref port) = pointer {
    if !inputs.contains(port) {
      result.add_error(
        format!(
          "Port {} from <op> is not defined in node {} inputs",
          port, node));
    }
  }
}

fn test_output(
    result: &mut CheckResult, 
    outputs: &HashSet<u32>,
    node: &Node,
    op: &Operation, 
    pointer: &ValuePointer) {
  if let &ValuePointer::PORT(ref port) = pointer {
    if !outputs.contains(port) {
      result.add_error(
        format!(
          "Port {} from <op> is not defined in node {} outputs",
          port, node));
    }
  }
}

fn check_node(node: &NodeBlock, result: &mut CheckResult) {
  let inputs = collect_input_ports(&node.1);
  let outputs = collect_output_ports(&node.2);

  for op in &node.3 {
    match op {
      &Operation::MOV(ref from, ref to) => {
        test_input(result, &inputs, &node.0, op, from);
        test_output(result, &outputs, &node.0, op, to);
      },
      &Operation::ADD(ref value) => {
        test_output(result, &inputs, &node.0, op, value);
      },
      &Operation::SUB(ref value) => {
        test_output(result, &inputs, &node.0, op, value);
      },
      &Operation::JRO(ref value) => {
        test_output(result, &inputs, &node.0, op, value);
      },
      _ => {}
    }
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

  #[test]
  fn test_check_node_on_JRO() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![],
      vec![
        Operation::JRO(ValuePointer::PORT(1)),
        Operation::JRO(ValuePointer::ACC),
        Operation::JRO(ValuePointer::VALUE(2))
      ]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![
        OutputMapping {
          from: 2,
          to: Port::new(Node::Out, 3),
        }
      ],
      vec![
        Operation::JRO(ValuePointer::PORT(2))
      ]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.has_errors(), true);
  }

  #[test]
  fn test_check_node_on_ADD() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![],
      vec![
        Operation::ADD(ValuePointer::PORT(1)),
        Operation::ADD(ValuePointer::ACC),
        Operation::ADD(ValuePointer::VALUE(2))
      ]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![
        OutputMapping {
          from: 2,
          to: Port::new(Node::Out, 3),
        }
      ],
      vec![
        Operation::ADD(ValuePointer::PORT(2))
      ]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.has_errors(), true);
  }

  #[test]
  fn test_check_node_on_SUB() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![],
      vec![
        Operation::SUB(ValuePointer::PORT(1)),
        Operation::SUB(ValuePointer::ACC),
        Operation::SUB(ValuePointer::VALUE(2))
      ]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![
        OutputMapping {
          from: 2,
          to: Port::new(Node::Out, 3),
        }
      ],
      vec![
        Operation::SUB(ValuePointer::PORT(2))
      ]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.has_errors(), true);
  }

  #[test]
  fn test_check_node_on_MOV() {
    let mut check = CheckResult::new();
    
    let node_ok = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 3),
          to: 1
        }
      ],
      vec![
        OutputMapping {
          from: 2,
          to: Port::new(Node::Out, 3)
        }
      ],
      vec![
        Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(2)),
        Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
        Operation::MOV(ValuePointer::ACC, ValuePointer::VALUE(2))
      ]
    );
    check_node(&node_ok, &mut check);
    assert_eq!(check.has_errors(), false);
    
    let node_ko = (
      Node::new_node(&"a"),
      vec![
        InputMapping {
          from: Port::new(Node::In, 1),
          to: 1
        }
      ],
      vec![
        OutputMapping {
          from: 2,
          to: Port::new(Node::Out, 1),
        }
      ],
      vec![
        Operation::MOV(ValuePointer::PORT(2), ValuePointer::ACC),
        Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1))
      ]
    );
    check_node(&node_ko, &mut check);
    assert_eq!(check.error_count(), 2);
  }
}