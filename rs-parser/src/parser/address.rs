use std::fmt;

use nom::IResult;

use parser::common::{be_uint, to_string, Input};

#[derive(PartialEq)]
pub enum Node {
	In,
	Out,
	Node(String),
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
			_ => panic!("Not a named node: {}", self),
		}
	}

	fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Node::In => write!(f, "<IN>"),
			&Node::Out => write!(f, "<OUT>"),
			&Node::Node(ref id) => write!(f, "Node#{}", id),
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Port {
	pub node: Node,
	pub port: u32,
}

impl Port {
	pub fn new(node: Node, port: u32) -> Self {
		Port {
			node: node,
			port: port,
		}
	}

	#[cfg(test)]
	pub fn named_port(node_name: &str, port: u32) -> Self {
		Port {
			node: Node::new_node(node_name),
			port: port,
		}
	}
}

fn input_node(input: Input) -> IResult<Input, Node> {
	let (remaining, _) = nom::bytes::complete::tag("IN")(input)?;
	Ok((remaining, Node::In))
}

fn output_node(input: Input) -> IResult<Input, Node> {
	let (remaining, _) = nom::bytes::complete::tag("OUT")(input)?;
	Ok((remaining, Node::Out))
}

fn node_id(input: Input) -> IResult<Input, Node> {
	// do_parse!(
	// 	tag!("#") >>
	// 	id: map_res!(
	// 		take_while!(is_alphanumeric),
	// 		to_string
	// 	) >>
	// 	(Node::Node(id))
	// )
	todo!()
}

pub fn node_ref(input: Input) -> IResult<Input, Node> {
	nom::branch::alt((input_node, output_node, node_id))(input)
}

pub fn port_ref(input: Input) -> IResult<Input, Port> {
	todo!()
	// do_parse!(
	// 	id: node_ref >>
	// 	tag!(":") >>
	// 	port: be_uint >>
	// 	(Port::new(id, port))
	// )
}

pub fn node_header(input: Input) -> IResult<Input, Node> {
	todo!()
	// do_parse!(
	// 	tag!("Node") >>
	// 	opt!(space) >>
	// 	id: node_id >>
	// 	(id)
	// )
}

#[cfg(test)]
mod tests {
	use super::*;
	use parser::common::tests::*;
	use parser::common::to_input;

	#[test]
	fn test_parse_input_node() {
		let content = to_input(b"IN aa");
		let res = input_node(content);
		assert_result(res, Node::In, to_input(b" aa"));
	}

	#[test]
	fn test_parse_output_node() {
		let content = to_input(b"OUT aa");
		let res = output_node(content);
		assert_result(res, Node::Out, to_input(b" aa"));
	}

	#[test]
	fn test_parse_node_id() {
		let content = to_input(b"#abc42");
		let res = node_id(content);
		assert_full_result(res, Node::new_node(&"abc42"));
	}

	#[test]
	fn test_parse_node_header() {
		let content = to_input(b"Node #a1");

		let res = node_header(content);
		assert_full_result(res, Node::new_node(&"a1"));
	}

	#[test]
	fn test_parse_node_ref() {
		let res_node = node_ref(to_input(b"#ref"));
		assert_full_result(res_node, Node::new_node(&"ref"));

		let res_in = node_ref(to_input(b"IN"));
		assert_full_result(res_in, Node::In);

		let res_out = node_ref(to_input(b"OUT"));
		assert_full_result(res_out, Node::Out);
	}

	#[test]
	fn test_parse_port_ref() {
		let res = port_ref(to_input(b"IN:13"));
		assert_full_result(res, Port::new(Node::In, 13));
	}
}
