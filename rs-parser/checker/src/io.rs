use std::collections::HashSet;

use crate::CheckResult;
use language::address::Node;
use language::syntax::NodeBlock;
use language::syntax::Program;

fn dups_to_str<T: std::fmt::Display>(duplicates: HashSet<T>) -> String {
    duplicates.iter().fold(String::new(), |mut acc, value| {
        acc.push_str(format!("{},", value).as_str());
        acc
    })
}

/// Checks that all ports from 1 to max are used,
/// otherwise, we are having holes in our input/output array.
/// This returns the list of ports not used.
///
/// # Arguments
/// * `ports` - set of used ports
fn check_ranges<T: std::hash::Hash + Eq + Ord + From<u8>>(
    ports: &HashSet<T>,
) -> Option<HashSet<T>> {
    ports.iter().max().and_then(|max_port| {
        if *max_port > (ports.len() as u8).into() {
            let missing_ports = (1..=255)
                .map(|i| i.into())
                .filter(|v| *v < *max_port)
                .filter(|v| !ports.contains(v))
                .collect();
            Some(missing_ports)
        } else {
            None
        }
    })
}

fn check_inputs(nodes: &[NodeBlock], result: &mut CheckResult) {
    let mut input_ports = HashSet::new();
    let mut duplicates = HashSet::new();
    for node in nodes {
        let inputs = &node.1;
        for input in inputs {
            let node = &input.from.node;
            let port = input.from.port;
            if node == &Node::In && !input_ports.insert(port) {
                duplicates.insert(port);
            }
        }
    }

    if !duplicates.is_empty() {
        result.add_error(format!(
            "Duplicated uses of input ports {}",
            dups_to_str(duplicates)
        ));
    }

    if let Some(ununsed_ports) = check_ranges(&input_ports) {
        result.add_warning(format!(
            "Unused ports in the input: {}",
            dups_to_str(ununsed_ports)
        ));
    }
}

fn check_outputs(nodes: &[NodeBlock], result: &mut CheckResult) {
    let mut output_ports = HashSet::new();
    let mut duplicates = HashSet::new();
    for node in nodes {
        let outputs = &node.2;
        for output in outputs {
            let node = &output.to.node;
            let port = output.to.port;
            if node == &Node::Out && !output_ports.insert(port) {
                duplicates.insert(port);
            }
        }
    }

    if !duplicates.is_empty() {
        result.add_error(format!(
            "Duplicated uses of output ports {}",
            dups_to_str(duplicates)
        ));
    }

    if let Some(ununsed_ports) = check_ranges(&output_ports) {
        result.add_warning(format!(
            "Unused ports in the output: {}",
            dups_to_str(ununsed_ports)
        ));
    }
}

pub fn check(tree: &Program, result: &mut CheckResult) -> bool {
    let initial_count = result.error_count();
    check_inputs(&tree.nodes, result);
    check_outputs(&tree.nodes, result);

    result.error_count() == initial_count
}

#[cfg(test)]
mod tests {
    use super::*;

    use language::address::Port;
    use language::syntax::{InputMapping, OutputMapping};

    #[test]
    fn test_check_in_ok() {
        let mut checks = Default::default();
        let nodes = vec![
            (
                Node::new_node(&"a"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 1.into(),
                    },
                    to: 1.into(),
                }],
                vec![],
                vec![],
            ),
            (
                Node::new_node(&"b"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 2.into(),
                    },
                    to: 2.into(),
                }],
                vec![],
                vec![],
            ),
        ];
        check_inputs(&nodes, &mut checks);
        assert_eq!(checks.has_errors(), false);
    }

    #[test]
    fn test_check_in_ko() {
        let mut checks = Default::default();
        let nodes = vec![
            (
                Node::new_node(&"a"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 3.into(),
                    },
                    to: 1.into(),
                }],
                vec![],
                vec![],
            ),
            (
                Node::new_node(&"b"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 3.into(),
                    },
                    to: 2.into(),
                }],
                vec![],
                vec![],
            ),
        ];
        check_inputs(&nodes, &mut checks);
        assert_eq!(checks.has_errors(), true);
    }

    #[test]
    fn test_check_out_ok() {
        let mut checks = Default::default();
        let nodes = vec![
            (
                Node::new_node(&"a"),
                vec![],
                vec![OutputMapping {
                    from: 1.into(),
                    to: Port {
                        node: Node::Out,
                        port: 1.into(),
                    },
                }],
                vec![],
            ),
            (
                Node::new_node(&"b"),
                vec![],
                vec![OutputMapping {
                    from: 2.into(),
                    to: Port {
                        node: Node::Out,
                        port: 2.into(),
                    },
                }],
                vec![],
            ),
        ];
        check_outputs(&nodes, &mut checks);
        assert_eq!(checks.has_errors(), false);
    }

    #[test]
    fn test_check_out_ko() {
        let mut checks = Default::default();
        let nodes = vec![
            (
                Node::new_node(&"a"),
                vec![],
                vec![OutputMapping {
                    from: 1.into(),
                    to: Port {
                        node: Node::Out,
                        port: 3.into(),
                    },
                }],
                vec![],
            ),
            (
                Node::new_node(&"b"),
                vec![],
                vec![OutputMapping {
                    from: 2.into(),
                    to: Port {
                        node: Node::Out,
                        port: 3.into(),
                    },
                }],
                vec![],
            ),
        ];
        check_outputs(&nodes, &mut checks);
        assert_eq!(checks.has_errors(), true);
    }

    #[test]
    fn test_check_complete() {
        let mut checks = Default::default();
        let nodes = vec![
            (
                Node::new_node(&"a"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 3.into(),
                    },
                    to: 1.into(),
                }],
                vec![OutputMapping {
                    from: 1.into(),
                    to: Port {
                        node: Node::Out,
                        port: 3.into(),
                    },
                }],
                vec![],
            ),
            (
                Node::new_node(&"b"),
                vec![InputMapping {
                    from: Port {
                        node: Node::In,
                        port: 3.into(),
                    },
                    to: 2.into(),
                }],
                vec![OutputMapping {
                    from: 2.into(),
                    to: Port {
                        node: Node::Out,
                        port: 3.into(),
                    },
                }],
                vec![],
            ),
        ];
        let tree = Program { nodes, tests: None };
        check(&tree, &mut checks);
        assert_eq!(checks.has_errors(), true);
        assert_eq!(checks.error_count(), 2);
    }

    #[test]
    fn test_warnings_about_unused_inputs() {
        let mut checks = Default::default();
        let nodes = vec![(
            Node::new_node(&"a"),
            vec![InputMapping {
                from: Port {
                    node: Node::In,
                    port: 3.into(),
                },
                to: 1.into(),
            }],
            vec![],
            vec![],
        )];
        let tree = Program { nodes, tests: None };
        check(&tree, &mut checks);
        assert_eq!(checks.has_warnings(), true);
    }

    #[test]
    fn test_warnings_about_unused_outputs() {
        let mut checks = Default::default();
        let nodes = vec![(
            Node::new_node(&"a"),
            vec![],
            vec![OutputMapping {
                from: 1.into(),
                to: Port {
                    node: Node::Out,
                    port: 3.into(),
                },
            }],
            vec![],
        )];
        let tree = Program { nodes, tests: None };
        check(&tree, &mut checks);
        assert_eq!(checks.has_warnings(), true);
    }
}
