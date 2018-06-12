use nom::{space};

use parser::common::{RawData, be_uint, ospace, eol, opt_eol};
use parser::address::{Node, Port, node_header, port_ref};
use parser::instruction::{parse_instruction, Operation};
use parser::instruction::condition::label_operation;

#[derive(Debug, PartialEq)]
pub struct InputMapping {
	pub from: Port,
	pub to: u32
}
#[derive(Debug, PartialEq)]
pub struct OutputMapping {
	pub from: u32,
	pub to: Port
}

// Syntax lines
named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));

// List of inputs
named!(input_item<&RawData, InputMapping>,
	do_parse!(
		port: port_ref >>
		space >> tag!("->") >> space >>
		input: be_uint >>
		(InputMapping { from: port, to: input })
	)
);
named!(inputs<&RawData, Vec<InputMapping> >,
	separated_list_complete!(
		do_parse!(
			ospace >> tag!(",") >> space >> ()
		),
		input_item
	)
);

// List of outputs
named!(output_item<&RawData, OutputMapping>,
	do_parse!(
		input: be_uint >>
		space >> tag!("->") >> space >>
		port: port_ref >>
		(OutputMapping { from: input, to: port })
	)
);
named!(outputs<&RawData, Vec<OutputMapping> >,
	separated_list_complete!(
		do_parse!(
			ospace >> tag!(",") >> space >>
			()
		),
		output_item
	)
);

named!(instruction_line<&RawData, Vec<Operation> >,
	alt!(
		// Instruction only
		do_parse!(
			op: parse_instruction >> eol >>
			(vec![op])
		) |
		// Label only
		do_parse!(
			label: label_operation >> eol >>
			(vec![label])
		) |
		// Label then instruction
		do_parse!(
			label: label_operation >> ospace >>
			op: parse_instruction >> eol >>
			(vec![label, op])
		) |
		// Nothing but empty lines
		value!(vec![], eol)
	)
);
named!(instruction_list<&RawData, Vec<Operation> >,
	fold_many1!(instruction_line, Vec::new(), |mut acc: Vec<_>, ops| {
		for op in ops {
    	acc.push(op);
		}
    acc
	})
);

pub type NodeBlock = (Node, Vec<InputMapping>, Vec<OutputMapping>, Vec<Operation>);
named!(node_block<&RawData, NodeBlock>,
	do_parse!(
		ospace >>
		node: node_header >> eol >>
		node_line >> eol >>
		opt_eol >>
		inputs: opt!(
			do_parse!(
				ospace >> is: inputs >> eol >>
				code_line >> eol >>
				(is)
			)
		) >>
		ops: instruction_list >>
		outputs: opt!(
			do_parse!(
				code_line >> eol >>
				ospace >> os: outputs >> eol >>
				(os)
			)
		) >>
		node_line >> eol >>
		(node, inputs.unwrap_or(vec![]), outputs.unwrap_or(vec![]), ops)
	)
);

named!(pub node_list<&RawData, Vec<NodeBlock> >,
	do_parse!(
		opt_eol >>
		list: separated_nonempty_list_complete!(opt_eol, node_block) >>
		opt_eol >>
		(list)
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::{assert_result, assert_full_result};
	use parser::instruction::{ValuePointer, MemoryPointer};

	#[test]
	fn test_parse_node_line() {
		let input = b"===\nrest";

		let res = node_line(input);
		assert_result(res, b"===", b"\nrest");
	}

	#[test]
	fn test_parse_code_line() {
		let input = b"----\nrest";

		let res = code_line(input);
		assert_result(res, b"----", b"\nrest");
	}

	#[test]
	fn test_parse_input_item() {
		let res_in = input_item(b"IN:1 -> 3");
		assert_full_result(
			res_in,
			InputMapping {
				from: Port::new(Node::In, 1),
				to: 3u32
			}
		);

		let res_node = input_item(b"#node:32 -> 1");
		assert_full_result(
			res_node,
			InputMapping {
				from: Port::named_port(&"node", 32),
				to: 1u32
			}
		);
	}

	#[test]
	fn test_parse_inputs() {
		let res_one = inputs(b"#n:7 -> 14");
		assert_full_result(
			res_one,
			vec![
				InputMapping {
					from: Port::named_port(&"n", 7),
					to: 14u32,
				}
			]
		);

		let res_many = inputs(b"OUT:1 -> 2, #abc:3 -> 4");
		assert_full_result(
			res_many,
			vec![
				InputMapping {
					from: Port::new(Node::Out, 1),
					to: 2u32
				},
				InputMapping {
					from: Port::named_port(&"abc", 3),
					to: 4u32
				}
			]
		);
	}

	#[test]
	fn test_parse_output_item() {
		let res_in = output_item(b"1 -> OUT:3");
		assert_full_result(
			res_in,
			OutputMapping {
				from: 1u32,
				to: Port::new(Node::Out, 3)
			}
		);

		let res_node = output_item(b"1 -> #node:32");
		assert_full_result(
			res_node,
			OutputMapping {
				from: 1u32,
				to: Port::named_port(&"node", 32)
			}
		);
	}

	#[test]
	fn test_parse_outputs() {
		let res_one = outputs(b"3 -> #n:7");
		assert_full_result(
			res_one,
			vec![
				OutputMapping {
					from: 3,
					to: Port::named_port(&"n", 7)
				}
			]
		);

		let res_many = outputs(b"1 -> OUT:2, 3 -> #abc:4");
		assert_full_result(
			res_many,
			vec![
				OutputMapping {
					from: 1u32,
					to: Port::new(Node::Out, 2)
				},
				OutputMapping {
					from: 3u32,
					to: Port::named_port(&"abc", 4)
				}
			]
		);
	}

	#[test]
	fn test_parse_instruction_line_with_label_only() {
		let res = instruction_line(b"LBL:  \n");
		assert_full_result(
			res,
			vec![Operation::LABEL(String::from("LBL"))]
		);
	}

	#[test]
	fn test_parse_instruction_line_with_instruction_only() {
		let res = instruction_line(b"SWP  \n");
		assert_full_result(
			res,
			vec![Operation::SWP(MemoryPointer::BAK(1))]
		);
	}

	#[test]
	fn test_parse_instruction_line_with_label_then_instruction() {
		let res = instruction_line(b"LBL:SWP \n");
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("LBL")),
				Operation::SWP(MemoryPointer::BAK(1))
			]
		);
	}

	#[test]
	fn test_parse_empty_instruction_line() {
		let res = instruction_line(b" \n");
		assert_full_result(res, vec![]);
	}

	#[test]
	fn test_parse_instruction_line_with_comment() {
		let res = instruction_line(b" // only comment\n");
		assert_full_result(res, vec![]);
	}

	#[test]
	fn test_parse_with_consecutive_labels() {
		let res = instruction_line(b"L1: L2:\n");
		assert!(res.is_err(), true);
	}

	#[test]
	fn test_parse_instruction_with_comment() {
		let res = instruction_line(b"ADD <2 // Sum the values\n");
		assert_full_result(
			res,
			vec![
				Operation::ADD(ValuePointer::PORT(2))
			]
		);
	}

	#[test]
	fn test_parse_label_and_instruction_with_comment() {
		let res = instruction_line(b"LBL: SUB <3 // Sum the values\n");
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("LBL")),
				Operation::SUB(ValuePointer::PORT(3))
			]
		);
	}

	#[test]
	fn test_parse_instruction_list() {
		let input = b"START:
MOV <1, ACC
F1:SWP
MOV ACC, >1
JEZ F1\n";
		let res = instruction_list(input);
		assert_full_result(
			res,
			vec![
				Operation::LABEL(String::from("START")),
				Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
				Operation::LABEL(String::from("F1")),
				Operation::SWP(MemoryPointer::BAK(1)),
				Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				Operation::JEZ(String::from("F1"))
			]
		);

	}

	#[test]
	fn test_parse_node_block() {
		let input = b"  Node #123
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

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("123"),
				vec![
					InputMapping {
						from: Port::new(Node::In, 1),
						to: 1
					}
				],
				vec![
					OutputMapping {
						from: 1,
						to: Port::new(Node::Out, 1)
					}
				],
				vec![
					Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
					Operation::SWP(MemoryPointer::BAK(1)),
					Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1))
				]
			)
		);
	}

	#[test]
	fn test_parse_node_without_mapping() {
		let input = b"  Node #123
==========
SWP
=======
";

		let res =  node_block(input);
		let (_, (_, res_inputs, res_outputs, _)) = res.unwrap();
		assert_eq!(res_inputs, vec![]);
		assert_eq!(res_outputs, vec![]);
	}

	#[test]
	fn test_parse_node_with_instruction_within_comments() {
		let input = b"Node #1
==========
// before
SWP
// after
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![
					Operation::SWP(MemoryPointer::BAK(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_node_with_instruction_and_eol_comment() {
		let input = b"Node #1
==========
SWP // commenting operation
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![
					Operation::SWP(MemoryPointer::BAK(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_node_with_indented_comment() {
		let input = b"Node #3
==========
  // indent
SWP
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("3"),
				vec![],
				vec![],
				vec![
					Operation::SWP(MemoryPointer::BAK(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_node_with_comments_before_intructions() {
		let input = b"Node #1
==========
// comment before
 // indented comment
SWP
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![
					Operation::SWP(MemoryPointer::BAK(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_node_with_comments_after_intructions() {
		let input = b"Node #1
==========
SWP
 // indented comment
// after instruction
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![],
				vec![],
				vec![
					Operation::SWP(MemoryPointer::BAK(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_commented_node() {
		let input = b"Node #1
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

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::new_node("1"),
				vec![
					InputMapping {
						from: Port::named_port(&"1", 1),
						to: 1
					},
					InputMapping {
						from: Port::named_port(&"2", 1),
						to: 2
					}
				],
				vec![
					OutputMapping {
						from: 1,
						to: Port::new(Node::Out, 1)
					}
				],
				vec![
					Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
					Operation::ADD(ValuePointer::PORT(2)),
					Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				]
			)
		);
	}

	#[test]
	fn test_parse_node_list() {
		let input = b"
 Node #1
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

		let res = node_list(input);
		assert_full_result(
			res,
			vec![
				(
					Node::new_node("1"),
					vec![
						InputMapping {
							from: Port::new(Node::In, 1),
							to: 1
						}
					],
					vec![
						OutputMapping {
							from: 1,
							to: Port::named_port(&"2", 2)
						}
					],
					vec![
						Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1)),
					]
				),
				(
					Node::new_node("2"),
					vec![
						InputMapping {
							from: Port::named_port(&"1", 1),
							to: 2
						}
					],
					vec![
						OutputMapping {
							from: 2,
							to: Port::named_port(&"3", 3)
						}
					],
					vec![
						Operation::MOV(ValuePointer::PORT(2), ValuePointer::PORT(2)),
					]
				),
				(
					Node::new_node("3"),
					vec![
						InputMapping {
							from: Port::named_port(&"2", 2),
							to: 3
						}
					],
					vec![
						OutputMapping {
							from: 3,
							to: Port::new(Node::Out, 1)
						}
					],
					vec![
						Operation::MOV(ValuePointer::PORT(3), ValuePointer::PORT(3)),
					]
				)
			]
		);

	}

}
