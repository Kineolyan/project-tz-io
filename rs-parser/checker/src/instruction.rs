use std::collections::HashSet;

use crate::CheckResult;
use language::address::{InputSlot, Node, OutputSlot};
use language::instruction::{Operation, ValuePointer};
use language::syntax::Program;
use language::syntax::{InputMapping, NodeBlock, OutputMapping};

/// Module checking that the ports referenced by instructions
/// are defined in the inputs/outputs.
/// This only generate warnings.

fn collect_input_ports(inputs: &[InputMapping]) -> HashSet<InputSlot> {
    inputs
        .iter()
        .map(|ref input| input.to)
        .collect::<HashSet<InputSlot>>()
}

fn collect_output_ports(outputs: &[OutputMapping]) -> HashSet<OutputSlot> {
    outputs
        .iter()
        .map(|ref output| output.from)
        .collect::<HashSet<OutputSlot>>()
}

fn test_input(
    result: &mut CheckResult,
    inputs: &HashSet<InputSlot>,
    node: &Node,
    op: &Operation,
    pointer: &ValuePointer,
) {
    if let ValuePointer::INPUT(ref port) = pointer {
        if !inputs.contains(port) {
            result.add_error(format!(
                "Port {} from {} is not defined in node {} inputs",
                port, op, node
            ));
        }
    }
}

fn test_output(
    result: &mut CheckResult,
    outputs: &HashSet<OutputSlot>,
    node: &Node,
    op: &Operation,
    pointer: &ValuePointer,
) {
    if let ValuePointer::OUTPUT(ref port) = pointer {
        if !outputs.contains(port) {
            result.add_error(format!(
                "Port {} from {} is not defined in node {} outputs",
                port, op, node
            ));
        }
    }
}

fn check_node(node: &NodeBlock, result: &mut CheckResult) {
    let inputs = collect_input_ports(&node.1);
    let outputs = collect_output_ports(&node.2);

    for op in &node.3 {
        match op {
            Operation::MOV(ref from, ref to) => {
                test_input(result, &inputs, &node.0, op, from);
                test_output(result, &outputs, &node.0, op, to);
            }
            Operation::ADD(ref value) => {
                test_input(result, &inputs, &node.0, op, value);
            }
            Operation::SUB(ref value) => {
                test_input(result, &inputs, &node.0, op, value);
            }
            Operation::JRO(ref value) => {
                test_input(result, &inputs, &node.0, op, value);
            }
            _ => {}
        }
    }
}

pub fn check(tree: &Program, result: &mut CheckResult) -> bool {
    let initial_count = result.error_count();
    for node in &tree.nodes {
        check_node(node, result);
    }

    initial_count == result.error_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use language::address::Port;

    #[test]
    fn test_check_node_on_jro() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![],
            vec![
                Operation::JRO(ValuePointer::INPUT(1.into())),
                Operation::JRO(ValuePointer::ACC),
                Operation::JRO(ValuePointer::VALUE(2)),
            ],
        );
        check_node(&node_ok, &mut check);
        assert_eq!(check.has_errors(), false);

        let node_ko = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 2.into(),
                to: Port::new(Node::Out, 3.into()),
            }],
            vec![Operation::JRO(ValuePointer::INPUT(2.into()))],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.has_errors(), true);
    }

    #[test]
    fn test_check_node_on_add() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![],
            vec![
                Operation::ADD(ValuePointer::INPUT(1.into())),
                Operation::ADD(ValuePointer::ACC),
                Operation::ADD(ValuePointer::VALUE(2)),
            ],
        );
        check_node(&node_ok, &mut check);
        assert_eq!(check.has_errors(), false);

        let node_ko = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 2.into(),
                to: Port::new(Node::Out, 3.into()),
            }],
            vec![Operation::ADD(ValuePointer::INPUT(2.into()))],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.has_errors(), true);
    }

    #[test]
    fn test_check_node_on_sub() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![],
            vec![
                Operation::SUB(ValuePointer::INPUT(1.into())),
                Operation::SUB(ValuePointer::ACC),
                Operation::SUB(ValuePointer::VALUE(2)),
            ],
        );
        check_node(&node_ok, &mut check);
        assert_eq!(check.has_errors(), false);

        let node_ko = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 2.into(),
                to: Port::new(Node::Out, 3.into()),
            }],
            vec![Operation::SUB(ValuePointer::INPUT(2.into()))],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.has_errors(), true);
    }

    #[test]
    fn test_check_node_on_mov() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 3.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 2.into(),
                to: Port::new(Node::Out, 3.into()),
            }],
            vec![
                Operation::MOV(
                    ValuePointer::INPUT(1.into()),
                    ValuePointer::OUTPUT(2.into()),
                ),
                Operation::MOV(ValuePointer::INPUT(1.into()), ValuePointer::ACC),
                Operation::MOV(ValuePointer::ACC, ValuePointer::VALUE(2)),
            ],
        );
        check_node(&node_ok, &mut check);
        assert_eq!(check.has_errors(), false);

        let node_ko = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port::new(Node::In, 1.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 2.into(),
                to: Port::new(Node::Out, 1.into()),
            }],
            vec![
                Operation::MOV(ValuePointer::INPUT(2.into()), ValuePointer::ACC),
                Operation::MOV(ValuePointer::ACC, ValuePointer::OUTPUT(1.into())),
            ],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.error_count(), 2);
    }
}
