use language::instruction::{MemoryPointer, Operation};

pub fn swp_operation(input: &[u8]) -> nom::IResult<&[u8], Operation> {
    nom::combinator::value(
        Operation::SWP(MemoryPointer::BAK(1)),
        nom::bytes::complete::tag("SWP"),
    )(input)
}

pub fn sav_operation(input: &[u8]) -> nom::IResult<&[u8], Operation> {
    nom::combinator::value(
        Operation::SAV(MemoryPointer::BAK(1)),
        nom::bytes::complete::tag("SAV"),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::tests::*;
    use crate::common::to_input;

    #[test]
    fn test_parse_swp_operation() {
        let res = swp_operation(to_input(b"SWP"));
        assert_full_result(res, Operation::SWP(MemoryPointer::BAK(1)));
    }

    #[test]
    fn test_parse_swp_operation_to_idx() {
        // let res = swp_operation(to_input(b"SWP 3"));
        // assert_full_result(res, Operation::SWP(MemoryPointer::BAK(3)));
    }

    #[test]
    fn test_parse_sav_operation() {
        let res = sav_operation(to_input(b"SAV"));
        assert_full_result(res, Operation::SAV(MemoryPointer::BAK(1)));
    }

    #[test]
    fn test_parse_sav_operation_to_idx() {
        // TODO code save to other space
        // let res = sav_operation(to_input(b"SAV 2"));
        // assert_full_result(res, Operation::SAV(MemoryPointer::BAK(2)));
    }
}
