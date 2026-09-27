use crate::common::Res;
use nom::Parser;
use nom_language::error::VerboseError;
use crate::common;
use language::instruction::{MemoryPointer, ValuePointer};
use nom::bytes::complete::tag;
use nom::combinator as c;

pub fn acc_pointer(input: &[u8]) -> Res<'_, ValuePointer> {
    c::value(ValuePointer::ACC, tag("ACC")).parse(input)
}

pub fn nil_pointer(input: &[u8]) -> Res<'_, ValuePointer> {
    c::value(ValuePointer::NIL, tag("NIL")).parse(input)
}

fn pointer<'a>(
    arrow: &'static str,
) -> impl Parser<&'a [u8], Output = u8, Error = VerboseError<&'a [u8]>> {
    nom::sequence::preceded(tag(arrow), common::be_u8)
}

pub fn input_pointer(input: &[u8]) -> Res<'_, ValuePointer> {
    c::map(pointer("<"), |slot| ValuePointer::INPUT(slot.into())).parse(input)
}

pub fn output_pointer(input: &[u8]) -> Res<'_, ValuePointer> {
    c::map(pointer(">"), |slot| ValuePointer::OUTPUT(slot.into())).parse(input)
}

pub fn value_pointer(input: &[u8]) -> Res<'_, ValuePointer> {
    c::map(common::be_uint, ValuePointer::VALUE).parse(input)
}

#[allow(dead_code)]
pub fn bak_pointer(input: &[u8]) -> Res<'_, MemoryPointer> {
    c::value(MemoryPointer::BAK(1), tag("BAK")).parse(input)
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
