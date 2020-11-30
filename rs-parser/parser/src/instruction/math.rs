use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::IResult;

use crate::instruction::base::{acc_pointer, input_pointer, nil_pointer, value_pointer};
use language::instruction::Operation;

pub fn add_operation(input: &[u8]) -> IResult<&[u8], Operation> {
	let (input, _) = tag("ADD")(input)?;
	let (input, _) = space1(input)?;
	let (input, value) = alt((input_pointer, acc_pointer, nil_pointer, value_pointer))(input)?;
	Ok((input, Operation::ADD(value)))
}

pub fn sub_operation(input: &[u8]) -> IResult<&[u8], Operation> {
	let (input, _) = tag("SUB")(input)?;
	let (input, _) = space1(input)?;
	let (input, value) = alt((input_pointer, acc_pointer, nil_pointer, value_pointer))(input)?;
	Ok((input, Operation::SUB(value)))
}

pub fn neg_operation(input: &[u8]) -> IResult<&[u8], Operation> {
	use nom::combinator::value;
	value(Operation::NEG, tag("NEG"))(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::common::tests::*;
	use crate::common::to_input;
	use language::instruction::ValuePointer;

	#[test]
	fn test_parse_add_operation_with_value() {
		let res = add_operation(to_input(b"ADD 1"));
		assert_full_result(res, Operation::ADD(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_add_operation_with_input() {
		let res = add_operation(to_input(b"ADD <17"));
		assert_full_result(res, Operation::ADD(ValuePointer::PORT(17)));
	}

	#[test]
	fn test_parse_add_operation_with_acc() {
		let res = add_operation(to_input(b"ADD ACC"));
		assert_full_result(res, Operation::ADD(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_add_operation_with_nil() {
		let res = add_operation(to_input(b"ADD NIL"));
		assert_full_result(res, Operation::ADD(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_add_from_out() {
		let res = add_operation(to_input(b"ADD >1"));
		assert_cannot_parse(res);
	}

	#[test]
	fn test_parse_sub_operation_with_value() {
		let res = sub_operation(to_input(b"SUB 1"));
		assert_full_result(res, Operation::SUB(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_sub_operation_with_input() {
		let res = sub_operation(to_input(b"SUB <17"));
		assert_full_result(res, Operation::SUB(ValuePointer::PORT(17)));
	}

	#[test]
	fn test_parse_sub_operation_with_acc() {
		let res = sub_operation(to_input(b"SUB ACC"));
		assert_full_result(res, Operation::SUB(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_sub_operation_with_nil() {
		let res = sub_operation(to_input(b"SUB NIL"));
		assert_full_result(res, Operation::SUB(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_sub_from_out() {
		let res = add_operation(to_input(b"SUB >1"));
		assert_cannot_parse(res);
	}

	#[test]
	fn test_parse_neg_operation() {
		let res = neg_operation(to_input(b"NEG"));
		assert_full_result(res, Operation::NEG);
	}
}
