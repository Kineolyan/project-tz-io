use std::fmt;

use nom::{space, is_alphanumeric};

use parser::common::{RawData, be_uint, to_string};

#[derive(PartialEq)]
pub enum Node {
	In,
	Out,
	Node(String)
}

impl fmt::Debug for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl Node {
	#[cfg(test)]
	pub fn new_node(name: &str) -> Self {
		Node::Node(name.to_string())
	}

	pub fn get_id<'a>(&'a self) -> &'a String {
		match self {
			&Node::Node(ref id) => id,
			_ => panic!("Not a named node: {}", self)
		}
	}

	fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Node::In => write!(f, "<IN>"),
      &Node::Out => write!(f, "<OUT>"),
      &Node::Node(ref id) => write!(f, "Node#{}", id)
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Port {
	pub node: Node,
	pub port: u32
}

impl Port {
	pub fn new(node: Node, port: u32) -> Self {
		Port { node: node, port: port }
	}

	#[cfg(test)]
	pub fn named_port(node_name: &str, port: u32) -> Self {
		Port { node: Node::new_node(node_name), port: port }
	}
}

named!(input_node<&RawData, Node>,
  do_parse!(tag!("IN") >> (Node::In))
);
named!(output_node<&RawData, Node>,
  do_parse!(tag!("OUT") >> (Node::Out))
);
named!(node_id<&RawData, Node>,
  do_parse!(
    tag!("#") >>
    id: map_res!(
      take_while!(is_alphanumeric),
      to_string
    ) >>
    (Node::Node(id))
  )
);
named!(pub node_ref<&RawData, Node>,
  alt!(input_node | output_node | node_id)
);

named!(pub port_ref<&RawData, Port>,
  do_parse!(
    id: node_ref >>
    tag!(":") >>
    port: be_uint >>
    (Port::new(id, port))
  )
);

named!(pub node_header<&RawData, Node>,
  do_parse!(
    tag!("Node") >>
    opt!(space) >>
    id: node_id >>
    (id)
  )
);

#[cfg(test)]
mod tests {
	use super::*;
	use parser::common::tests::{assert_result, assert_full_result};

	#[test]
	fn test_parse_input_node() {
		let input = b"IN aa";
		let res = input_node(input);
		assert_result(res, Node::In, b" aa");
	}

	#[test]
	fn test_parse_output_node() {
		let input = b"OUT aa";
		let res = output_node(input);
		assert_result(res, Node::Out, b" aa");
	}

	#[test]
	fn test_parse_node_id() {
		let input = b"#abc42";
		let res = node_id(input);
		assert_full_result(res, Node::new_node(&"abc42"));
	}

	#[test]
	fn test_parse_node_header() {
		let input = b"Node #a1";

		let res = node_header(input);
		assert_full_result(res, Node::new_node(&"a1"));
	}

	#[test]
	fn test_parse_node_ref() {
		let res_node = node_ref(b"#ref");
		assert_full_result(res_node, Node::new_node(&"ref"));

		let res_in = node_ref(b"IN");
		assert_full_result(res_in, Node::In);

		let res_out = node_ref(b"OUT");
		assert_full_result(res_out, Node::Out);
	}

	#[test]
	fn test_parse_port_ref() {
		let res = port_ref(b"IN:13");
		assert_full_result(res, Port::new(Node::In, 13));
	}
}
