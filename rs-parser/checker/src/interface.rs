use std::collections::HashSet;

use crate::CheckResult;
use language::syntax::NodeBlock;
use language::syntax::Program;

/// Module checking that the ports referenced by inputs
/// or outputs for duplicated port numbers.

fn check_ports<T, U, F: Fn(&T) -> U>(inputs: &[T], accessor: F) -> HashSet<U>
where
    U: Copy + std::hash::Hash + Eq,
{
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

fn dups_to_str<T: std::fmt::Display>(duplicates: HashSet<T>) -> String {
    duplicates.iter().fold(String::new(), |mut acc, value| {
        acc.push_str(format!("{},", value).as_str());
        acc
    })
}

fn check_node(node: &NodeBlock, result: &mut CheckResult) {
    let input_duplicates = check_ports(&node.1, |ref input| input.to);
    if !input_duplicates.is_empty() {
        result.add_error(format!(
            "Duplicated input ports referenced in {}: {}",
            &node.0,
            dups_to_str(input_duplicates)
        ));
    }

    let output_duplicates = check_ports(&node.2, |ref output| output.from);
    if !output_duplicates.is_empty() {
        result.add_error(format!(
            "Duplicated output ports referenced in {}: {}",
            &node.0,
            dups_to_str(output_duplicates)
        ));
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

    use language::address::{Node, Port};
    use language::syntax::{InputMapping, OutputMapping};

    fn fake_input(i: u8) -> InputMapping {
        InputMapping {
            from: Port::new(Node::In, i.into()),
            to: i.into(),
        }
    }

    #[test]
    fn test_check_input_duplicates() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![fake_input(1), fake_input(2), fake_input(3)],
            vec![],
            vec![],
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
                fake_input(3),
            ],
            vec![],
            vec![],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.has_errors(), true);
    }

    fn fake_output(i: u8) -> OutputMapping {
        OutputMapping {
            from: i.into(),
            to: Port::new(Node::Out, i.into()),
        }
    }

    #[test]
    fn test_check_output_duplicates() {
        let mut check = Default::default();

        let node_ok = (
            Node::new_node(&"a"),
            vec![],
            vec![fake_output(1), fake_output(2), fake_output(3)],
            vec![],
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
                fake_output(3),
            ],
            vec![],
        );
        check_node(&node_ko, &mut check);
        assert_eq!(check.has_errors(), true);
    }
}
