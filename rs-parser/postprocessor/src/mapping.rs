use std::collections::HashMap;

use language::address::{InputSlot, Node, OutputSlot, Port};
use language::syntax::Program;
use language::syntax::{InputMapping, NodeBlock, OutputMapping};

/// TODO this is perform inside a single operation, we could use &str instead of a String
type Index = HashMap<String, usize>;
fn map_node_to_idx(tree: &Program) -> Index {
    let mut index = HashMap::new();
    for (i, &(ref node, _, _, _)) in tree.nodes.iter().enumerate() {
        if let Node::Node(ref node_id) = node {
            index.insert(node_id.clone(), i);
        }
    }
    index
}

/// Complete nodes inputs with the outputs referenced by nodes
fn complete_inputs(mut tree: Program, index: &Index) -> Program {
    let mut additions = Vec::new();
    for node in tree.nodes.iter() {
        let this_id = &node.0.get_id();
        // Read outputs and add them to their sources
        let outputs: &Vec<OutputMapping> = &node.2;
        for output in outputs.iter() {
            if let Node::Node(ref dst_id) = &output.to.node {
                let idx = index
                    .get(dst_id)
                    .unwrap_or_else(|| panic!("No reference to node {}", dst_id));
                let dst_node = &tree.nodes[*idx];
                // this output m: i -> n:j => input n: m:i -> j
                let addtional_input =
                    complete_input(dst_node, this_id, output.from, output.to.port);
                if let Some(input) = addtional_input {
                    additions.push((*idx, input));
                }
            }
        }
    }

    for (i, input) in additions {
        let node = &mut tree.nodes[i];
        node.1.push(input);
    }

    tree
}

fn complete_input(
    node: &NodeBlock,
    src_id: &str,
    from: OutputSlot,
    to: InputSlot,
) -> Option<InputMapping> {
    // Skip if the port is already present
    let inputs: &Vec<InputMapping> = &node.1;
    if !inputs.iter().any(|input| match input.from.node {
        Node::Node(ref id) => id == src_id && input.from.port == from,
        _ => false,
    }) {
        Some(InputMapping {
            from: Port {
                node: Node::Node(src_id.to_owned()),
                port: from,
            },
            to,
        })
    } else {
        None
    }
}

/// Complete nodes outputs with the inputs referenced by nodes
fn complete_outputs(mut tree: Program, index: &Index) -> Program {
    let mut additions = Vec::new();
    for node in tree.nodes.iter() {
        let this_id = &node.0.get_id();
        // Read inputs and add them to the source
        let inputs: &Vec<InputMapping> = &node.1;
        for input in inputs.iter() {
            if let Node::Node(ref dst_id) = input.from.node {
                let idx = index
                    .get(dst_id)
                    .unwrap_or_else(|| panic!("No reference to node {}", dst_id));
                let src_node = &tree.nodes[*idx];
                let addtional_output =
                    complete_output(src_node, this_id, input.from.port, input.to);
                if let Some(o) = addtional_output {
                    additions.push((*idx, o));
                }
            }
        }
    }

    for (i, output) in additions {
        let node = &mut tree.nodes[i];
        node.2.push(output);
    }

    tree
}

fn complete_output(
    node: &NodeBlock,
    dst_id: &str,
    from: OutputSlot,
    to: InputSlot,
) -> Option<OutputMapping> {
    // Skip if the port is already present
    let outputs: &Vec<OutputMapping> = &node.2;
    if !outputs.iter().any(|output| match output.to.node {
        Node::Node(ref id) => id == dst_id && output.to.port == to,
        _ => false,
    }) {
        Some(OutputMapping {
            from,
            to: Port {
                node: Node::Node(dst_id.to_owned()),
                port: to,
            },
        })
    } else {
        None
    }
}

pub fn complete_mappings(tree: Program) -> Program {
    let nodes = map_node_to_idx(&tree);
    let tree = complete_inputs(tree, &nodes);
    complete_outputs(tree, &nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    use language::address::Port;
    use language::syntax::{InputMapping, OutputMapping};

    #[test]
    fn test_complete_node_inputs() {
        let src = (
            Node::new_node(&"a"),
            vec![],
            vec![
                OutputMapping {
                    from: 1.into(),
                    to: Port {
                        node: Node::new_node(&"b"),
                        port: 2.into(),
                    },
                },
                OutputMapping {
                    from: 2.into(),
                    to: Port {
                        node: Node::Out,
                        port: 1.into(),
                    },
                },
            ],
            vec![],
        );
        let dst = (Node::new_node(&"b"), vec![], vec![], vec![]);
        let tree = complete_mappings(Program {
            nodes: vec![src, dst],
            tests: None,
        });
        assert_eq!(
            tree.nodes[1].1,
            vec![InputMapping {
                from: Port {
                    node: Node::new_node(&"a"),
                    port: 1.into()
                },
                to: 2.into()
            }]
        );
    }

    #[test]
    fn test_complete_node_outputs() {
        let src = (Node::new_node(&"a"), vec![], vec![], vec![]);
        let dst = (
            Node::new_node(&"b"),
            vec![
                InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 1.into(),
                    },
                    to: 1.into(),
                },
                InputMapping {
                    from: Port {
                        node: Node::new_node(&"a"),
                        port: 1.into(),
                    },
                    to: 2.into(),
                },
            ],
            vec![],
            vec![],
        );
        let tree = complete_mappings(Program {
            nodes: vec![src, dst],
            tests: None,
        });
        assert_eq!(
            tree.nodes[0].2,
            vec![OutputMapping {
                from: 1.into(),
                to: Port {
                    node: Node::new_node(&"b"),
                    port: 2.into()
                }
            }]
        );
    }

    #[test]
    fn test_complete_partial_definitions() {
        let src = (
            Node::new_node(&"a"),
            vec![],
            vec![OutputMapping {
                from: 2.into(),
                to: Port {
                    node: Node::new_node(&"b"),
                    port: 2.into(),
                },
            }],
            vec![],
        );
        let dst = (
            Node::new_node(&"b"),
            vec![InputMapping {
                from: Port {
                    node: Node::new_node(&"a"),
                    port: 1.into(),
                },
                to: 1.into(),
            }],
            vec![],
            vec![],
        );
        let tree = complete_mappings(Program {
            nodes: vec![src, dst],
            tests: None,
        });
        assert_eq!(
            tree.nodes[0].2,
            vec![
                OutputMapping {
                    from: 2.into(),
                    to: Port {
                        node: Node::new_node(&"b"),
                        port: 2.into()
                    }
                },
                OutputMapping {
                    from: 1.into(),
                    to: Port {
                        node: Node::new_node(&"b"),
                        port: 1.into()
                    }
                }
            ]
        );
        assert_eq!(
            tree.nodes[1].1,
            vec![
                InputMapping {
                    from: Port {
                        node: Node::new_node(&"a"),
                        port: 1.into()
                    },
                    to: 1.into()
                },
                InputMapping {
                    from: Port {
                        node: Node::new_node(&"a"),
                        port: 2.into()
                    },
                    to: 2.into()
                }
            ]
        );
    }
}
