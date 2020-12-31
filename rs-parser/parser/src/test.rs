use crate::common::ws;
use language::{
    address::{InputSlot, OutputSlot},
    test::TestCase,
};
use nom::bytes::complete as bytes;
use nom::character::complete::space0;
use nom::IResult;

pub fn values(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
    nom::multi::separated_list1(nom::character::complete::space1, crate::common::be_i8)(input)
}

pub fn array(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
    nom::sequence::delimited(bytes::tag("["), values, bytes::tag("]"))(input)
}

fn test_values<'a, Slot>(
    tag: &'static str,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (Slot, Vec<i8>)>
where
    Slot: From<u8>,
{
    move |input| {
        let (input, _) = bytes::tag(tag)(input)?;
        // TODO at this point, we are in a test comment, the syntax must be correct
        let (input, (slot, _)) = ws(nom::sequence::tuple((
            crate::common::be_u8,
            bytes::tag(":"),
        )))(input)?;
        let (input, values) = ws(array)(input)?;
        let (rest, _) = nom::sequence::tuple((space0, bytes::tag("\n")))(input)?;
        Ok((rest, (slot.into(), values)))
    }
}

fn test_input_values(input: &[u8]) -> IResult<&[u8], (OutputSlot, Vec<i8>)> {
    test_values("/>> ")(input)
}

fn test_output_values(input: &[u8]) -> IResult<&[u8], (InputSlot, Vec<i8>)> {
    test_values("/<< ")(input)
}

pub fn test_case(input: &[u8]) -> IResult<&[u8], TestCase> {
    let mut test: Option<TestCase> = None;
    let mut remaining = input;
    loop {
        if let Ok((rest, (input_slot, input_values))) = test_input_values(remaining) {
            test = test
                .or_else(|| Some(Default::default()))
                .map(|t| t.input_into(input_slot, input_values));
            remaining = rest;
            continue;
        }
        if let Ok((rest, (output_slot, output_values))) = test_output_values(remaining) {
            test = test
                .or_else(|| Some(Default::default()))
                .map(|t| t.output_from(output_slot, output_values));
            remaining = rest;
            continue;
        }

        return test.map_or_else(
            || {
                Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Satisfy,
                )))
            },
            |t| Ok((remaining, t)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;
    use crate::common::to_input;

    #[test]
    fn test_parse_values() {
        let res = values(to_input(b"-1 2  3 4"));
        assert_full_result(res, vec![-1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_array_squares() {
        let res = array(to_input(b"[1 -2 3]"));
        assert_full_result(res, vec![1, -2, 3]);
    }

    #[test]
    fn test_parse_array_simple() {
        let res = array(to_input(b"10 5 2"));
        assert_cannot_parse(res);
    }

    #[test]
    fn test_parse_array_mixed() {
        let open_res = array(to_input(b"[10 5 2"));
        assert_cannot_parse(open_res);

        // Valid as an array with trailing ]
        let close_res = array(to_input(b"10 5 2]"));
        assert_cannot_parse(close_res);
    }

    #[test]
    fn test_parse_input_test_case() {
        let res = test_case(to_input(b"/>> 4: [1 2]  \nnext"));
        assert_result(
            res,
            TestCase::default().input_into(4.into(), vec![1, 2]),
            to_input(b"next"),
        );
    }

    #[test]
    fn test_parse_unclosed_input_test_case() {
        let res = test_case(to_input(b"/>> 1: [1 2  \nnext"));
        assert_cannot_parse(res);
    }

    #[test]
    fn test_parse_opposite_input_test_case() {
        let res = test_case(to_input(b"/>> 3: 1,2]\nnext"));
        assert_cannot_parse(res);
    }

    #[test]
    fn test_parse_output_test_case() {
        let res = test_case(to_input(b"/<< 4: [1 2]  \nnext"));
        assert_result(
            res,
            TestCase::default().output_from(4.into(), vec![1, 2]),
            to_input(b"next"),
        );
    }

    #[test]
    fn test_parse_unclosed_output_test_case() {
        let res = test_case(to_input(b"/<< 1: [1 2  \nnext"));
        assert_cannot_parse(res);
    }

    #[test]
    fn test_parse_opposite_output_test_case() {
        let res = test_case(to_input(b"/<< 3: 1,2]\nnext"));
        assert_cannot_parse(res);
    }

    #[test]
    fn test_parse_input_and_output_test_cases() {
        let res = test_case(
            b"/<< 1: [101 102]
/>> 1: [11 12 13]
/>> 3: [31 32 33]
/<< 2: [127]
// after",
        );
        assert_result(
            res,
            TestCase::default()
                .input_into(1.into(), vec![11, 12, 13])
                .input_into(3.into(), vec![31, 32, 33])
                .output_from(1.into(), vec![101, 102])
                .output_from(2.into(), vec![127]),
            b"// after",
        );
    }
}
