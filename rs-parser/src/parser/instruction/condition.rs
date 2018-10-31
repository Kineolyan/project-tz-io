use nom::{alphanumeric, space};

use parser::common::{Input, ospace, to_string};
use parser::instruction::Operation;
use parser::instruction::base::{
	acc_pointer,
	nil_pointer,
	input_pointer,
	value_pointer
};

named!(label_name<Input, String>,
	map_res!(alphanumeric, to_string)
);

named!(pub label_operation<Input, Operation>,
	do_parse!(
		label: label_name >> ospace >> tag!(":") >>
		(Operation::LABEL(label))
	)
);

// JMP, JEZ, JNZ, JGZ, JLZ, JRO
macro_rules! jump_fn {
	($name:ident, $pattern:expr, $cnstr:path) => {
		named!(pub $name<Input, Operation>,
			do_parse!(
				tag!($pattern) >> space >>
				label: label_name >>
				($cnstr(label))
			)
		);
	};
}
jump_fn!(jmp_operation, "JMP", Operation::JMP);
jump_fn!(jez_operation, "JEZ", Operation::JEZ);
jump_fn!(jnz_operation, "JNZ", Operation::JNZ);
jump_fn!(jlz_operation, "JLZ", Operation::JLZ);
jump_fn!(jgz_operation, "JGZ", Operation::JGZ);

named!(pub jro_operation<Input, Operation>,
	do_parse!(
		tag!("JRO") >> space >>
		value: alt!(acc_pointer | nil_pointer | input_pointer | value_pointer) >>
		(Operation::JRO(value))
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::*;
	use parser::instruction::ValuePointer;

	#[test]
	fn test_parse_label_operation() {
		let res = label_operation(input(b"aLabel1:"));
		assert_full_result(res, Operation::LABEL(String::from("aLabel1")));
	}

	#[test]
	fn test_parse_label_operation_with_space() {
		let res = label_operation(input(b"spaceLbl  :"));
		assert_full_result(res, Operation::LABEL(String::from("spaceLbl")));
	}

	#[test]
	fn test_parse_label_operation_with_next() {
		let res = label_operation(input(b"lbl: NEG"));
		assert_result(res, Operation::LABEL(String::from("lbl")), input(b" NEG"));
	}

	#[test]
	fn test_parse_jmp_operation() {
		let res = jmp_operation(input(b"JMP label"));
		assert_full_result(res, Operation::JMP(String::from("label")));
	}

	#[test]
	fn test_parse_jez_operation() {
		let res = jez_operation(input(b"JEZ label"));
		assert_full_result(res, Operation::JEZ(String::from("label")));
	}

	#[test]
	fn test_parse_jnz_operation() {
		let res = jnz_operation(input(b"JNZ label"));
		assert_full_result(res, Operation::JNZ(String::from("label")));
	}

	#[test]
	fn test_parse_jlz_operation() {
		let res = jlz_operation(input(b"JLZ label"));
		assert_full_result(res, Operation::JLZ(String::from("label")));
	}

	#[test]
	fn test_parse_jgz_operation() {
		let res = jgz_operation(input(b"JGZ label"));
		assert_full_result(res, Operation::JGZ(String::from("label")));
	}

	#[test]
	fn test_parse_jro_operation_with_value() {
		let res = jro_operation(input(b"JRO 1"));
		assert_full_result(res, Operation::JRO(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_jro_operation_with_input() {
		let res = jro_operation(input(b"JRO <32"));
		assert_full_result(res, Operation::JRO(ValuePointer::PORT(32)));
	}

	#[test]
	fn test_parse_jro_operation_with_acc() {
		let res = jro_operation(input(b"JRO ACC"));
		assert_full_result(res, Operation::JRO(ValuePointer::ACC));
	}

	#[test]
	fn test_parse_jro_operation_with_nil() {
		let res = jro_operation(input(b"JRO NIL"));
		assert_full_result(res, Operation::JRO(ValuePointer::NIL));
	}

	#[test]
	fn test_cannot_parse_jro_operation_from_out() {
		let res = jro_operation(input(b"JRO >12"));
		assert_cannot_parse(res);
	}
}