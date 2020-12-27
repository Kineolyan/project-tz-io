use std::collections::HashMap;

use crate::result::CheckResult;
use language::address::Node;
use language::syntax::NodeBlock;
use language::syntax::Program;

/// Module checking that the mappings between the various nodes
/// are consistent.
/// For example, when a node A maps its outputs to node B, if B
/// defines its inputs, A and B must map the same ports.
/// A.out: [1 -> B#1] and B.in: [A#1 -> 1] are ok, same mapping
/// A.out: [1 -> B#1] and B.in: [A#2 -> 2] are ok, they is no overlap
/// A.out: [1 -> B#1] and B.in: [A#2 -> 1] are inconsistent

type Index<'a> = HashMap<&'a String, usize>;
// TODO move this method to some utility module
fn map_node_to_idx<'a>(nodes: &'a [NodeBlock], index: &mut Index<'a>) {
    for (i, &(ref node, _, _, _)) in nodes.iter().enumerate() {
        if let Node::Node(ref node_id) = node {
            index.insert(node_id, i);
        }
    }
}

fn check_node_inputs(
    result: &mut CheckResult,
    node: &NodeBlock,
    nodes: &[NodeBlock],
    index: &Index,
) {
    let this_id = match &node.0 {
        Node::Node(ref id) => id,
        _ => panic!("Node of incorrect type"),
    };
    let inputs = &node.1;
    for input in inputs.iter() {
        if let Node::Node(ref src_id) = &input.from.node {
            let is_match = index
                .get(src_id)
                .map(|node_idx| &nodes[*node_idx])
                .map(|ref src_node| {
                    src_node.2.iter().any(|ref output|
            // Output m: i -> n:j <=> Input n: m:i -> j
            match &output.to.node {
              Node::Node(ref id) =>
                id == this_id
                && output.from == input.from.port
                && output.to.port == input.to,
              _ => false
            })
                })
                .unwrap_or(false);
            if !is_match {
                // TODO code display for input
                result.add_error(format!(
                    "No corresponding output for input {} of node {}",
                    "<in>", /*input*/ this_id
                ));
            }
        }
    }
}

fn check_node_outputs(
    result: &mut CheckResult,
    node: &NodeBlock,
    nodes: &[NodeBlock],
    index: &Index,
) {
    let this_id = match &node.0 {
        Node::Node(ref id) => id,
        _ => panic!("Node of incorrect type"),
    };
    let outputs = &node.2;
    for output in outputs.iter() {
        if let Node::Node(ref src_id) = &output.to.node {
            let is_match = index
                .get(src_id)
                .map(|node_idx| &nodes[*node_idx])
                .map(|ref dst_node| {
                    dst_node.1.iter().any(|ref input|
            // Output m: i -> n:j <=> Input n: m:i -> j
            match &input.from.node {
              Node::Node(ref id) =>
                id == this_id
                && input.from.port == output.from
                && input.to == output.to.port,
              _ => false
            })
                })
                .unwrap_or(false);
            if !is_match {
                // TODO code display for input
                result.add_error(format!(
                    "No corresponding output for input {} of node {}",
                    "<in>", /*input*/ this_id
                ));
            }
        }
    }
}

pub fn check(tree: &Program, result: &mut CheckResult) -> bool {
    let mut index = HashMap::new();
    {
        map_node_to_idx(&tree.nodes, &mut index);
    }

    let initial_count = result.error_count();
    for node in tree.nodes.iter() {
        check_node_inputs(result, node, &tree.nodes, &index);
        check_node_outputs(result, node, &tree.nodes, &index);
    }

    result.error_count() == initial_count
}

#[cfg(test)]
mod tests {
    use super::*;

    use language::address::Port;
    use language::syntax::{InputMapping, OutputMapping};

    #[test]
    fn test_check_valid_mappings() {
        let mut check_result = Default::default();

        let src = (
            Node::new_node(&"a"),
            vec![],
            vec![OutputMapping {
                from: 1,
                to: Port {
                    node: Node::new_node(&"b"),
                    port: 2,
                },
            }],
            vec![],
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
            vec![],
            vec![],
        );
        let nodes = vec![src, dst];
        let tree = Program {
            nodes,
            tests: None,
        };
        let result = check(&tree, &mut check_result);
        assert_eq!(result, true);
        assert_eq!(check_result.has_errors(), false);
        let mut check_result = Default::default();

        let src = (
            Node::new_node(&"a"),
            vec![],
            vec![OutputMapping {
                from: 1,
                to: Port {
                    node: Node::new_node(&"b"),
                    port: 2,
                },
            }],
            vec![],
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
            vec![],
            vec![],
        );
        let nodes = vec![src, dst];
        let tree = Program {
            nodes,
            tests: None,
        };
        let result = check(&tree, &mut check_result);
        assert_eq!(result, true);
        assert_eq!(check_result.has_errors(), false);
    }

    #[test]
    fn test_check_invalid_mappings() {
        let mut check_result = Default::default();

        let src = (
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 1,
                },
                to: 1,
            }],
            vec![
                OutputMapping {
                    from: 1,
                    to: Port {
                        node: Node::new_node(&"b"),
                        port: 3, // Incorrect port
                    },
                },
                OutputMapping {
                    from: 4, // Incorrect port
                    to: Port {
                        node: Node::new_node(&"b"),
                        port: 2,
                    },
                },
                OutputMapping {
                    from: 1,
                    to: Port {
                        node: Node::new_node(&"c"), // Incorrect name
                        port: 2,
                    },
                },
            ],
            vec![],
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
                from: 1,
                to: Port {
                    node: Node::Out,
                    port: 1,
                },
            }],
            vec![],
        );
        let tree = Program {
            nodes: vec![src, dst],
            tests: None,
        };
        let result = check(&tree, &mut check_result);
        assert_eq!(result, false);
        assert_eq!(check_result.error_count(), 4);
    }
}
