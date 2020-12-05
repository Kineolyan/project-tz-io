use language::address::{Node, Port};
use nom::IResult;

fn input_node(input: &[u8]) -> IResult<&[u8], Node> {
	let (remaining, _) = nom::bytes::complete::tag("IN")(input)?;
	Ok((remaining, Node::In))
}

fn output_node(input: &[u8]) -> IResult<&[u8], Node> {
	let (remaining, _) = nom::bytes::complete::tag("OUT")(input)?;
	Ok((remaining, Node::Out))
}

fn node_id(input: &[u8]) -> IResult<&[u8], Node> {
	let (input, _) = nom::bytes::complete::tag("#")(input)?;
	let (input, id) = nom::combinator::map_res(
		nom::bytes::complete::take_while(nom::character::is_alphanumeric),
		crate::common::to_string,
	)(input)?;
	Ok((input, Node::Node(id)))
}

pub fn node_ref(input: &[u8]) -> IResult<&[u8], Node> {
	nom::branch::alt((input_node, output_node, node_id))(input)
}

pub fn port_ref(input: &[u8]) -> IResult<&[u8], Port> {
	let (input, id) = node_ref(input)?;
	let (input, _) = nom::bytes::complete::tag(":")(input)?;
	let (input, port) = crate::common::be_uint(input)?;
	Ok((input, Port::new(id, port)))
}

pub fn node_header(input: &[u8]) -> IResult<&[u8], Node> {
	let (input, _) = nom::bytes::complete::tag("Node")(input)?;
	let (input, _) = nom::character::complete::space0(input)?;
	node_id(input)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::common::tests::*;
	use crate::common::to_input;

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
