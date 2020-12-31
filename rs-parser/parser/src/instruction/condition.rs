use nom::IResult;

use crate::common::to_string;
use crate::instruction::base::{acc_pointer, input_pointer, nil_pointer, value_pointer};
use language::instruction::Operation;

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
            // TODO once we see the operation, we can deduce that it must be followed by a label
            // Using Failure would mark the issue in the parsing
            let (rest, (_, _, label)) = nom::sequence::tuple((
                nom::bytes::complete::tag($pattern),
                nom::character::complete::space1,
                label_name,
            ))(input)?;
            Ok((rest, $cnstr(label)))
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
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, value) =
        nom::branch::alt((acc_pointer, nil_pointer, input_pointer, value_pointer))(input)?;
    Ok((input, Operation::JRO(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::tests::*;
    use crate::common::to_input;
    use language::instruction::ValuePointer;

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
        assert_full_result(res, Operation::JRO(ValuePointer::INPUT(32.into())));
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
