use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

type Index = HashMap<String, usize>;
fn map_node_to_idx<'a>(tree: &ParsingTree) -> Index {
  let mut index = HashMap::new();
  for (i, &(ref node, _, _, _)) in tree.iter().enumerate() {
    if let &Node::Node(ref node_id) = node {
      index.insert(node_id.clone(), i);
    }
  }
  index
}

/// Complete nodes inputs with the outputs referenced by nodes
fn complete_inputs(mut tree: ParsingTree, index: &Index) -> ParsingTree {
  let mut additions = Vec::new();
  for node in tree.iter() {
    let this_id = &node.0.get_id();
    // Read outputs and add them to their sources
    let outputs: &Vec<OutputMapping> = &node.2;
    for output in outputs.iter() {
      if let &Node::Node(ref dst_id) = &output.to.node {
        let idx = index.get(dst_id).unwrap_or_else(|| panic!("No reference to node {}", dst_id));
        let dst_node = &tree[*idx];
        // this output m: i -> n:j => input n: m:i -> j
        let addtional_input = complete_input(dst_node, this_id, output.from, output.to.port);
        if let Some(input) = addtional_input {
          additions.push((*idx, input));
        }
      }
    }
  }

  for (i, input) in additions {
    let node = &mut tree[i];
    node.1.push(input);
  }

  tree
}

fn complete_input(node: &NodeBlock, src_id: &String, from: u32, to: u32) -> Option<InputMapping> {
  // Skip if the port is already present
  let inputs: &Vec<InputMapping> = &node.1;
  if !inputs.iter().any(|input| match input.from.node {
    Node::Node(ref id) => id == src_id && input.from.port == from,
    _ => false
  }) {
    Some(InputMapping {
      from: Port {
        node: Node::Node(src_id.clone()),
        port: from
      },
      to: to
    })
  } else {
    None
  }
}

/// Complete nodes outputs with the inputs referenced by nodes
fn complete_outputs(mut tree: ParsingTree, index: &Index) -> ParsingTree{
  let mut additions = Vec::new();
  for node in tree.iter() {
    let this_id = &node.0.get_id();
    // Read inputs and add them to the source
    let inputs: &Vec<InputMapping> = &node.1;
    for input in inputs.iter() {
      if let Node::Node(ref dst_id) = input.from.node {
        let idx = index.get(dst_id).unwrap_or_else(|| panic!("No reference to node {}", dst_id));
        let src_node = &tree[*idx];
        let addtional_output = complete_output(src_node, this_id, input.from.port, input.to);
        if let Some(o) = addtional_output {
          additions.push((*idx, o));
        }
      }
    }
  }

  for (i, output) in additions {
    let node = &mut tree[i];
    node.2.push(output);
  }

  tree
}

fn complete_output(node: &NodeBlock, dst_id: &String, from: u32, to: u32) -> Option<OutputMapping> {
  // Skip if the port is already present
  let outputs: &Vec<OutputMapping> = &node.2;
  if !outputs.iter().any(|output| match output.to.node {
    Node::Node(ref id) => id == dst_id && output.to.port == to,
    _ => false
  }) {
    Some(OutputMapping {
      from: from,
      to: Port {
        node: Node::Node(dst_id.clone()),
        port: to
      }
    })
  } else {
    None
  }
}

pub fn complete_mappings(tree: ParsingTree) -> ParsingTree {
  let nodes = map_node_to_idx(&tree);

  let tree = complete_inputs(tree, &nodes);
  let tree = complete_outputs(tree, &nodes);

  tree
}

#[cfg(test)]
mod tests {
  use super::*;

  use parser::address::Port;
  use parser::syntax::{InputMapping, OutputMapping};

  #[test]
  fn test_complete_node_inputs() {
    let src = (
      Node::new_node(&"a"),
      vec![],
      vec![
        OutputMapping {
          from: 1,
          to: Port {
            node: Node::new_node(&"b"),
            port: 2
          }
        },
        OutputMapping {
          from: 2,
          to: Port {
            node: Node::Out,
            port: 1
          }
        }
      ],
      vec![]
    );
    let dst = (
      Node::new_node(&"b"),
      vec![],
      vec![],
      vec![]
    );
    let tree = complete_mappings(vec![src, dst]);
    assert_eq!(tree[1].1, vec![
      InputMapping {
        from: Port {
          node: Node::new_node(&"a"),
          port: 1
        },
        to: 2
      }
    ]);
  }

  #[test]
  fn test_complete_node_outputs() {
    let src = (
      Node::new_node(&"a"),
      vec![],
      vec![],
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
          to: 1
        },
        InputMapping {
          from: Port {
            node: Node::new_node(&"a"),
            port: 1
          },
          to: 2
        }
      ],
      vec![],
      vec![]
    );
    let tree = complete_mappings(vec![src, dst]);
    assert_eq!(tree[0].2, vec![
      OutputMapping {
        from: 1,
        to: Port {
          node: Node::new_node(&"b"),
          port: 2
        }
      }
    ]);
  }

  #[test]
  fn test_complete_partial_definitions() {
    let src = (
      Node::new_node(&"a"),
      vec![],
      vec![
        OutputMapping {
          from: 2,
          to: Port {
            node: Node::new_node(&"b"),
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
            node: Node::new_node(&"a"),
            port: 1
          },
          to: 1
        }
      ],
      vec![],
      vec![]
    );
    let tree = complete_mappings(vec![src, dst]);
    assert_eq!(tree[0].2, vec![
      OutputMapping {
        from: 2,
        to: Port {
          node: Node::new_node(&"b"),
          port: 2
        }
      },
      OutputMapping {
        from: 1,
        to: Port {
          node: Node::new_node(&"b"),
          port: 1
        }
      }
    ]);
    assert_eq!(tree[1].1, vec![
        InputMapping {
          from: Port {
            node: Node::new_node(&"a"),
            port: 1
          },
          to: 1
        },
        InputMapping {
          from: Port {
            node: Node::new_node(&"a"),
            port: 2
          },
          to: 2
        }
    ]);
  }

}