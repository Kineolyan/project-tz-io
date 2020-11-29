use nom::IResult;
use language::address::{Node, Port};

// use nom::number::complete::be_u32;
// use common::to_string;

fn input_node(input: &[u8]) -> IResult<&[u8], Node> {
	let (remaining, _) = nom::bytes::complete::tag("IN")(input)?;
	Ok((remaining, Node::In))
}

fn output_node(input: &[u8]) -> IResult<&[u8], Node> {
	let (remaining, _) = nom::bytes::complete::tag("OUT")(input)?;
	Ok((remaining, Node::Out))
}

fn node_id(input: &[u8]) -> IResult<&[u8], Node> {
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

pub fn node_ref(input: &[u8]) -> IResult<&[u8], Node> {
	nom::branch::alt((input_node, output_node, node_id))(input)
}

pub fn port_ref(input: &[u8]) -> IResult<&[u8], Port> {
	todo!()
	// do_parse!(
	// 	id: node_ref >>
	// 	tag!(":") >>
	// 	port: be_u32 >>
	// 	(Port::new(id, port))
	// )
}

pub fn node_header(input: &[u8]) -> IResult<&[u8], Node> {
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
