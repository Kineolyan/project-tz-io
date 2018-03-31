use nom::{space};

use parser::common::RawData;
use parser::instruction::Operation;
use parser::instruction::base::{
	value_pointer,
	input_pointer,
	acc_pointer,
	nil_pointer
};

named!(pub add_operation<&RawData, Operation>,
	do_parse!(
		tag!("ADD") >> space >>
		value: alt!(input_pointer | acc_pointer | nil_pointer | value_pointer) >>
		(Operation::ADD(value))
	)
);

named!(pub sub_operation<&RawData, Operation>,
	do_parse!(
		tag!("SUB") >> space >>
		value: alt!(input_pointer | acc_pointer | nil_pointer | value_pointer) >>
		(Operation::SUB(value))
	)
);

named!(pub neg_operation<&RawData, Operation>,
	value!(Operation::NEG, tag!("NEG"))
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::*;
	use parser::instruction::ValuePointer;

	#[test]
	fn test_parse_add_operation_with_value() {
		let res = add_operation(b"ADD 1");
		assert_full_result(res, Operation::ADD(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_add_operation_with_input() {
		let res = add_operation(b"ADD <17");
		assert_full_result(res, Operation::ADD(ValuePointer::PORT(17)));
	}

	#[test]
	fn test_parse_add_operation_with_acc() {
		let res = add_operation(b"ADD ACC");
		assert_full_result(res, Operation::ADD(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_add_operation_with_nil() {
		let res = add_operation(b"ADD NIL");
		assert_full_result(res, Operation::ADD(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_add_from_out() {
		let res = add_operation(b"ADD >1");
		assert_cannot_parse(res);
	}

	#[test]
	fn test_parse_sub_operation_with_value() {
		let res = sub_operation(b"SUB 1");
		assert_full_result(res, Operation::SUB(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_sub_operation_with_input() {
		let res = sub_operation(b"SUB <17");
		assert_full_result(res, Operation::SUB(ValuePointer::PORT(17)));
	}

	#[test]
	fn test_parse_sub_operation_with_acc() {
		let res = sub_operation(b"SUB ACC");
		assert_full_result(res, Operation::SUB(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_sub_operation_with_nil() {
		let res = sub_operation(b"SUB NIL");
		assert_full_result(res, Operation::SUB(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_sub_from_out() {
		let res = add_operation(b"SUB >1");
		assert_cannot_parse(res);
	}

	#[test]
	fn test_parse_neg_operation() {
		let res = neg_operation(b"NEG");
		assert_full_result(res, Operation::NEG);
	}

}