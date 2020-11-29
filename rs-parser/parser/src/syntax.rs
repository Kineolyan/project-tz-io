use nom;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::{space0, space1};
use nom::IResult; //space;
use nom::number::complete::be_u32;

use crate::address::{node_header, port_ref};
use crate::common::{eol, opt_eol};
// use crate::instruction::condition::label_operation;
// use crate::instruction::parse_instruction;
use language::instruction::Operation;
use language::syntax::{InputMapping, OutputMapping};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
// fn list_separator<T, Input, Error: nom::error::ParseError<&[u8]>>(sep: T) -> impl Fn(Input) -> IResult<&[u8], Input, Error>
// where
// 	Input: nom::&[u8]Take + nom::Compare<T>,
// 	T: nom::&[u8]Length + Clone {
//   nom::sequence::delimited(
//     space0,
//     tag(sep),
//     space0
//   )
// }

// Syntax lines
pub fn node_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
	take_while(|c| c == b'=')(input)
}
pub fn code_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
	take_while(|c| c == b'-')(input)
}

// List of inputs
pub fn input_item(input: &[u8]) -> IResult<&[u8], InputMapping> {
	let (remaining, (port, _, _, _, input)) =
		nom::sequence::tuple((port_ref, space0, tag("->"), space0, be_u32))(input)?;
	let mapping = InputMapping {
		from: port,
		to: input,
	};
	Ok((remaining, mapping))
}

fn input_separator(input: &[u8]) -> IResult<&[u8], ()> {
	let (input, _) = space0(input)?;
	let (input, _) = tag(",")(input)?;
	let (input, _) = space1(input)?;

	Ok((input, ()))
}

pub fn inputs(input: &[u8]) -> IResult<&[u8], Vec<InputMapping>> {
	nom::multi::separated_list1(input_separator, input_item)(input)
}

// List of outputs
pub fn output_item(input: &[u8]) -> IResult<&[u8], OutputMapping> {
	let (remaining, (input, _, _, _, port)) =
		nom::sequence::tuple((be_u32, space0, tag("->"), space0, port_ref))(input)?;
	let mapping = OutputMapping {
		from: input,
		to: port,
	};
	Ok((remaining, mapping))
}

pub fn outputs(input: &[u8]) -> IResult<&[u8], Vec<OutputMapping>> {
	nom::multi::separated_list1(input_separator, output_item)(input)
}

fn instruction_line(input: &[u8]) -> IResult<&[u8], Vec<Operation> > {
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

pub fn instruction_list(input: &[u8]) -> IResult<&[u8], Vec<Operation>> {
	// fold_many1!(instruction_line, Vec::new(), |mut acc: Vec<_>, ops| {
	// 	for op in ops {
	// 		acc.push(op);
	// 	}
	// 	acc
	// })
	todo!()
}

pub fn node_block(input: &[u8]) -> IResult<&[u8], language::syntax::NodeBlock> {
	todo!()
	// do_parse!(
	// 	space0 >>
	// 	node: node_header >> eol >>
	// 	node_line >> eol >>
	// 	opt_eol >>
	// 	inputs: opt!(
	// 		do_parse!(
	// 			space0 >> is: inputs >> eol >>
	// 			code_line >> eol >>
	// 			(is)
	// 		)
	// 	) >>
	// 	ops: instruction_list >>
	// 	outputs: opt!(
	// 		do_parse!(
	// 			code_line >> eol >>
	// 			space0 >> os: outputs >> eol >>
	// 			(os)
	// 		)
	// 	) >>
	// 	node_line >> eol >>
	// 	(node, inputs.unwrap_or(vec![]), outputs.unwrap_or(vec![]), ops)
	// )
}

pub fn node_list(input: &[u8]) -> IResult<&[u8], Vec<language::syntax::NodeBlock>> {
	nom::multi::separated_list1(opt_eol, node_block)(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::common::tests::*;
	use crate::common::to_input;
	use language::instruction::{MemoryPointer, ValuePointer};
use language::address::{Node, Port};

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
	fn test_parse_input_item() {
		let res_in = input_item(to_input(b"IN:1 -> 3"));
		assert_full_result(
			res_in,
			InputMapping {
				from: Port::new(Node::In, 1),
				to: 3u32,
			},
		);

		let res_node = input_item(to_input(b"#node:32 -> 1"));
		assert_full_result(
			res_node,
			InputMapping {
				from: Port::named_port(&"node", 32),
				to: 1u32,
			},
		);
	}

	#[test]
	fn test_parse_inputs() {
		let res_one = inputs(to_input(b"#n:7 -> 14"));
		assert_full_result(
			res_one,
			vec![InputMapping {
				from: Port::named_port(&"n", 7),
				to: 14u32,
			}],
		);

		let res_many = inputs(to_input(b"OUT:1 -> 2, #abc:3 -> 4"));
		assert_full_result(
			res_many,
			vec![
				InputMapping {
					from: Port::new(Node::Out, 1),
					to: 2u32,
				},
				InputMapping {
					from: Port::named_port(&"abc", 3),
					to: 4u32,
				},
			],
		);
	}

	#[test]
	fn test_parse_output_item() {
		let res_in = output_item(to_input(b"1 -> OUT:3"));
		assert_full_result(
			res_in,
			OutputMapping {
				from: 1u32,
				to: Port::new(Node::Out, 3),
			},
		);

		let res_node = output_item(to_input(b"1 -> #node:32"));
		assert_full_result(
			res_node,
			OutputMapping {
				from: 1u32,
				to: Port::named_port(&"node", 32),
			},
		);
	}

	#[test]
	fn test_parse_outputs() {
		let res_one = outputs(to_input(b"3 -> #n:7"));
		assert_full_result(
			res_one,
			vec![OutputMapping {
				from: 3,
				to: Port::named_port(&"n", 7),
			}],
		);

		let res_many = outputs(to_input(b"1 -> OUT:2, 3 -> #abc:4"));
		assert_full_result(
			res_many,
			vec![
				OutputMapping {
					from: 1u32,
					to: Port::new(Node::Out, 2),
				},
				OutputMapping {
					from: 3u32,
					to: Port::named_port(&"abc", 4),
				},
			],
		);
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
