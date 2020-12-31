use crate::common;
use language::instruction::{MemoryPointer, ValuePointer};
use nom::bytes::complete::tag;
use nom::combinator as c;
use nom::IResult;

pub fn acc_pointer(input: &[u8]) -> IResult<&[u8], ValuePointer> {
    c::value(ValuePointer::ACC, tag("ACC"))(input)
}

pub fn nil_pointer(input: &[u8]) -> IResult<&[u8], ValuePointer> {
    c::value(ValuePointer::NIL, tag("NIL"))(input)
}

fn pointer<'a>(arrow: &'static str) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], u8> {
    nom::sequence::preceded(tag(arrow), common::be_u8)
}

pub fn input_pointer(input: &[u8]) -> IResult<&[u8], ValuePointer> {
    c::map(pointer("<"), |slot| ValuePointer::INPUT(slot.into()))(input)
}

pub fn output_pointer(input: &[u8]) -> IResult<&[u8], ValuePointer> {
    c::map(pointer(">"), |slot| ValuePointer::OUTPUT(slot.into()))(input)
}

pub fn value_pointer(input: &[u8]) -> IResult<&[u8], ValuePointer> {
    c::map(common::be_uint, ValuePointer::VALUE)(input)
}

#[allow(dead_code)]
pub fn bak_pointer(input: &[u8]) -> IResult<&[u8], MemoryPointer> {
    c::value(MemoryPointer::BAK(1), tag("BAK"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;
    use crate::common::to_input;

    #[test]
    fn test_parse_acc_pointer() {
        let res_explicit = acc_pointer(to_input(b"ACC"));
        assert_full_result(res_explicit, ValuePointer::ACC);
    }

    #[test]
    fn test_parse_nil_pointer() {
        let res_explicit = nil_pointer(to_input(b"NIL"));
        assert_full_result(res_explicit, ValuePointer::NIL);
    }

    #[test]
    fn test_parse_input_pointer() {
        let res = input_pointer(to_input(b"<12"));
        assert_full_result(res, ValuePointer::INPUT(12.into()));
    }

    #[test]
    fn test_parse_output_pointer() {
        let res = output_pointer(to_input(b">43"));
        assert_full_result(res, ValuePointer::OUTPUT(43.into()));
    }

    #[test]
    fn test_parse_value_pointer() {
        let res = value_pointer(to_input(b"37"));
        assert_full_result(res, ValuePointer::VALUE(37u32));
    }

    #[test]
    fn test_parse_bak_pointer() {
        let res = bak_pointer(to_input(b"BAK"));
        assert_full_result(res, MemoryPointer::BAK(1));
    }
}
