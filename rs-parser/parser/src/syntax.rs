use nom;
use nom::bytes::complete::take_while;
use nom::IResult;

use crate::common::opt_eol;
// use crate::instruction::condition::label_operation;
// use crate::instruction::parse_instruction;
use language::instruction::Operation;
use language::syntax::{InputMapping, OutputMapping};

/// Line marking the start/end of a node
pub fn node_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
	take_while(|c| c == b'=')(input)
}
/// Line separating inputs/outputs from the node instructions
pub fn code_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
	take_while(|c| c == b'-')(input)
}

fn instruction_line(input: &[u8]) -> IResult<&[u8], Vec<Operation>> {
	// alt!(
	// 	// Instruction only
	// 	do_parse!(
	// 		op: parse_instruction >> eol >>
	// 		(vec![op])
	// 	) |
	// 	// Label only
	// 	do_parse!(
	// 		label: label_operation >> eol >>
	// 		(vec![label])
	// 	) |
	// 	// Label then instruction
	// 	do_parse!(
	// 		label: label_operation >> space0 >>
	// 		op: parse_instruction >> eol >>
	// 		(vec![label, op])
	// 	) |
	// 	// Nothing but empty lines
	// 	value!(vec![], eol)
	// )
	todo!()
}

fn fail(input: &[u8]) -> nom::Err<nom::error::Error<&[u8]>> {
	nom::Err::Failure(nom::error::Error::new(
		input,
		nom::error::ErrorKind::Satisfy,
	))
}

pub fn node_block(input: &[u8]) -> IResult<&[u8], language::syntax::NodeBlock> {
	let (input, node) = crate::address::node_header(input)?;
	// At this point, we must see the start of a block
	let (input, _) = node_line(input).map_err(|_| fail(input))?;
	let (input, _) = nom::character::complete::newline(input)?;
	let (input, inputs) = if let Ok((some, inputs)) = crate::mapping::inputs(input) {
		let (rest, _) = code_line(some).map_err(|_| fail(input))?;
		(rest, inputs)
	} else {
		(input, vec![])
	};
	// FIXME consume some possible comments before any instruction
	let mut instructions = vec![];
	let mut remaining = input;
	while let Ok((some, mut instruction)) = instruction_line(remaining) {
		let (rest, _) = crate::common::eol(some).map_err(|_| fail(input))?;
		instructions.append(&mut instruction);
		remaining = rest;
	}
	// FIXME we must parse the output mapping
	// 	ops: instruction_list >>
	// 	outputs: opt!(
	// 		do_parse!(
	// 			code_line >> eol >>
	// 			space0 >> os: outputs >> eol >>
	// 			(os)
	// 		)
	// 	) >>
	Ok((input, (node, inputs, vec![], instructions)))
}

pub fn node_list(input: &[u8]) -> IResult<&[u8], Vec<language::syntax::NodeBlock>> {
	nom::multi::separated_list1(opt_eol, node_block)(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::common::tests::*;
	use crate::common::to_input;
	use language::address::{Node, Port};
	use language::instruction::{MemoryPointer, ValuePointer};

	#[test]
	fn test_parse_node_line() {
		let content = to_input(b"===\nrest");

		let res = node_line(content);
		assert_result(res, to_input(b"==="), to_input(b"\nrest"));
	}

	#[test]
	fn test_parse_code_line() {
		let content = to_input(b"----\nrest");

		let res = code_line(content);
		assert_result(res, to_input(b"----"), to_input(b"\nrest"));
	}

	#[test]
	fn test_parse_instruction_line_with_label_only() {
		let res = instruction_line(to_input(b"LBL:  \n"));
		assert_full_result(res, vec![Operation::LABEL(String::from("LBL"))]);
	}

	#[test]
	fn test_parse_instruction_line_with_instruction_only() {
		let res = instruction_line(to_input(b"SWP  \n"));
		assert_full_result(res, vec![Operation::SWP(MemoryPointer::BAK(1))]);
	}

	#[test]
	fn test_parse_instruction_line_with_label_then_instruction() {
		let res = instruction_line(to_input(b"LBL:SWP \n"));
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("LBL")),
				Operation::SWP(MemoryPointer::BAK(1)),
			],
		);
	}

	#[test]
	fn test_parse_empty_instruction_line() {
		let res = instruction_line(to_input(b" \n"));
		assert_full_result(res, vec![]);
	}

	#[test]
	fn test_parse_instruction_line_with_comment() {
		let res = instruction_line(to_input(b" // only comment\n"));
		assert_full_result(res, vec![]);
	}

	#[test]
	fn test_parse_with_consecutive_labels() {
		let res = instruction_line(to_input(b"L1: L2:\n"));
		assert!(res.is_err(), true);
	}

	#[test]
	fn test_parse_instruction_with_comment() {
		let res = instruction_line(to_input(b"ADD <2 // Sum the values\n"));
		assert_full_result(res, vec![Operation::ADD(ValuePointer::PORT(2))]);
	}

	#[test]
	fn test_parse_label_and_instruction_with_comment() {
		let res = instruction_line(to_input(b"LBL: SUB <3 // Sum the values\n"));
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("LBL")),
				Operation::SUB(ValuePointer::PORT(3)),
			],
		);
	}

	#[test]
	fn test_parse_instruction_list() {
		let content = b"START:
MOV <1, ACC
F1:SWP
MOV ACC, >1
JEZ F1\n";
		let res = instruction_list(to_input(content));
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("START")),
				Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
				Operation::LABEL(String::from("F1")),
				Operation::SWP(MemoryPointer::BAK(1)),
				Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				Operation::JEZ(String::from("F1")),
			],
		);
	}

	#[test]
	fn test_parse_node_block() {
		let content = b"  Node #123
==========
IN:1 -> 1
--
MOV <1, ACC
SWP
MOV ACC, >1
---------
1 -> OUT:1
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("123"),
				vec![InputMapping {
					from: Port::new(Node::In, 1),
					to: 1,
				}],
				vec![OutputMapping {
					from: 1,
					to: Port::new(Node::Out, 1),
				}],
				vec![
					Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
					Operation::SWP(MemoryPointer::BAK(1)),
					Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				],
			),
		);
	}

	#[test]
	fn test_parse_node_without_mapping() {
		let content = b"  Node #123
==========
SWP
=======
";

		let res = node_block(to_input(content));
		let (_, (_, res_inputs, res_outputs, _)) = res.unwrap();
		assert_eq!(res_inputs, vec![]);
		assert_eq!(res_outputs, vec![]);
	}

	#[test]
	fn test_parse_node_with_instruction_within_comments() {
		let content = b"Node #1
==========
// before
SWP
// after
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![Operation::SWP(MemoryPointer::BAK(1))],
			),
		);
	}

	#[test]
	fn test_parse_node_with_instruction_and_eol_comment() {
		let content = b"Node #1
==========
SWP // commenting operation
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![Operation::SWP(MemoryPointer::BAK(1))],
			),
		);
	}

	#[test]
	fn test_parse_node_with_indented_comment() {
		let content = b"Node #3
==========
  // indent
SWP
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("3"),
				vec![],
				vec![],
				vec![Operation::SWP(MemoryPointer::BAK(1))],
			),
		);
	}

	#[test]
	fn test_parse_node_with_comments_before_intructions() {
		let content = b"Node #1
==========
// comment before
 // indented comment
SWP
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![Operation::SWP(MemoryPointer::BAK(1))],
			),
		);
	}

	#[test]
	fn test_parse_node_with_comments_after_intructions() {
		let content = b"Node #1
==========
SWP
 // indented comment
// after instruction
=======
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![Operation::SWP(MemoryPointer::BAK(1))],
			),
		);
	}

	#[test]
	fn test_parse_commented_node() {
		let content = b"Node #1
=======
// Possible to repeat the same source (for readability)
#1:1 -> 1, #2:1 -> 2
---------
MOV <1, ACC
ADD <2 // Sum the values
MOV ACC, >1
------------
1 -> OUT:1
=========
";

		let res = node_block(to_input(content));
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![
					InputMapping {
						from: Port::named_port(&"1", 1),
						to: 1,
					},
					InputMapping {
						from: Port::named_port(&"2", 1),
						to: 2,
					},
				],
				vec![OutputMapping {
					from: 1,
					to: Port::new(Node::Out, 1),
				}],
				vec![
					Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
					Operation::ADD(ValuePointer::PORT(2)),
					Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				],
			),
		);
	}

	#[test]
	fn test_parse_node_list() {
		let content = b"Node #1
==========
IN:1 -> 1
--
MOV <1,  >1
---------
1 -> #2:2
=======

 Node #2
==========
#1:1 -> 2
----------
MOV <2, >2
----------
2 -> #3:3
==========

Node #3
==========
#2:2 -> 3
----------
MOV <3, >3
----------
3 -> OUT:1
==========
";

		let res = node_list(to_input(content));
		assert_full_result(
			res,
			vec![
				(
					Node::new_node("1"),
					vec![InputMapping {
						from: Port::new(Node::In, 1),
						to: 1,
					}],
					vec![OutputMapping {
						from: 1,
						to: Port::named_port(&"2", 2),
					}],
					vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
				),
				(
					Node::new_node("2"),
					vec![InputMapping {
						from: Port::named_port(&"1", 1),
						to: 2,
					}],
					vec![OutputMapping {
						from: 2,
						to: Port::named_port(&"3", 3),
					}],
					vec![Operation::MOV(ValuePointer::PORT(2), ValuePointer::PORT(2))],
				),
				(
					Node::new_node("3"),
					vec![InputMapping {
						from: Port::named_port(&"2", 2),
						to: 3,
					}],
					vec![OutputMapping {
						from: 3,
						to: Port::new(Node::Out, 1),
					}],
					vec![Operation::MOV(ValuePointer::PORT(3), ValuePointer::PORT(3))],
				),
			],
		);
	}
}
