use nom;
use nom::character::complete::space1;
use nom::IResult;

use common::to_string;
use instruction::Operation;
use instruction::base::{
	value_pointer,
	input_pointer,
	acc_pointer,
	nil_pointer
};

fn label_name(input: &[u8]) -> IResult<&[u8], String> {
	nom::combinator::map_res(nom::character::complete::alphanumeric1, to_string)(input)
}

pub fn label_operation(input: &[u8]) -> IResult<&[u8], Operation> {
	let (input, (label, _, _)) = nom::sequence::tuple((
		label_name,
		nom::character::complete::space0,
		nom::bytes::complete::tag(":"),
	))(input)?;
	Ok((input, Operation::LABEL(label)))
}

// JMP, JEZ, JNZ, JGZ, JLZ, JRO
macro_rules! jump_fn {
	($name:ident, $pattern:expr, $cnstr:path) => {
		pub fn $name(input: &[u8]) -> IResult<&[u8], Operation> {
			// do_parse!(
			// 	tag!($pattern) >> space1 >>
			// 	label: label_name >>
			// 	($cnstr(label))
			// )
			todo!()
		}
	};
}
jump_fn!(jmp_operation, "JMP", Operation::JMP);
jump_fn!(jez_operation, "JEZ", Operation::JEZ);
jump_fn!(jnz_operation, "JNZ", Operation::JNZ);
jump_fn!(jlz_operation, "JLZ", Operation::JLZ);
jump_fn!(jgz_operation, "JGZ", Operation::JGZ);

pub fn jro_operation(input: &[u8]) -> IResult<&[u8], Operation> {
	let (input, _) = nom::bytes::complete::tag("JRO")(input)?;
	let (input, value) =
		nom::branch::alt((acc_pointer, nil_pointer, input_pointer, value_pointer))(input)?;
	Ok((input, Operation::JRO(value)))
}

#[cfg(test)]
mod tests {
	use super::*;

	use common::tests::*;
	use common::to_input;
	use instruction::ValuePointer;

	#[test]
	fn test_parse_label_operation() {
		let res = label_operation(to_input(b"aLabel1:"));
		assert_full_result(res, Operation::LABEL(String::from("aLabel1")));
	}

	#[test]
	fn test_parse_label_operation_with_space() {
		let res = label_operation(to_input(b"spaceLbl  :"));
		assert_full_result(res, Operation::LABEL(String::from("spaceLbl")));
	}

	#[test]
	fn test_parse_label_operation_with_next() {
		let res = label_operation(to_input(b"lbl: NEG"));
		assert_result(
			res,
			Operation::LABEL(String::from("lbl")),
			to_input(b" NEG"),
		);
	}

	#[test]
	fn test_parse_jmp_operation() {
		let res = jmp_operation(to_input(b"JMP label"));
		assert_full_result(res, Operation::JMP(String::from("label")));
	}

	#[test]
	fn test_parse_jez_operation() {
		let res = jez_operation(to_input(b"JEZ label"));
		assert_full_result(res, Operation::JEZ(String::from("label")));
	}

	#[test]
	fn test_parse_jnz_operation() {
		let res = jnz_operation(to_input(b"JNZ label"));
		assert_full_result(res, Operation::JNZ(String::from("label")));
	}

	#[test]
	fn test_parse_jlz_operation() {
		let res = jlz_operation(to_input(b"JLZ label"));
		assert_full_result(res, Operation::JLZ(String::from("label")));
	}

	#[test]
	fn test_parse_jgz_operation() {
		let res = jgz_operation(to_input(b"JGZ label"));
		assert_full_result(res, Operation::JGZ(String::from("label")));
	}

	#[test]
	fn test_parse_jro_operation_with_value() {
		let res = jro_operation(to_input(b"JRO 1"));
		assert_full_result(res, Operation::JRO(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_jro_operation_with_input() {
		let res = jro_operation(to_input(b"JRO <32"));
		assert_full_result(res, Operation::JRO(ValuePointer::PORT(32)));
	}

	#[test]
	fn test_parse_jro_operation_with_acc() {
		let res = jro_operation(to_input(b"JRO ACC"));
		assert_full_result(res, Operation::JRO(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_jro_operation_with_nil() {
		let res = jro_operation(to_input(b"JRO NIL"));
		assert_full_result(res, Operation::JRO(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_jro_operation_from_out() {
		let res = jro_operation(to_input(b"JRO >12"));
		assert_cannot_parse(res);
	}
}
