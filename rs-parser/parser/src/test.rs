use crate::common::ws;
use language::test::TestCase;
use nom::bytes::complete as bytes;
use nom::character::complete::space0;
use nom::IResult;

pub fn values(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
  nom::multi::separated_list1(ws(bytes::tag(",")), crate::common::be_i8)(input)
}

pub fn array(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
  nom::sequence::delimited(bytes::tag("["), values, bytes::tag("]"))(input)
}

pub fn test_case(input: &[u8]) -> IResult<&[u8], TestCase> {
  let (input, _) = bytes::tag("///")(input)?;
  let (input, _) = space0(input)?;

  // TODO at this point, we are in a test comment, the syntax must be correct
  let (input, inputs) = array(input)?;
  let (input, _) = ws(bytes::tag("->"))(input)?;
  let (input, outputs) = array(input)?;
  let (rest, _) = nom::sequence::tuple((space0, bytes::tag("\n")))(input)?;
  Ok((rest, (TestCase::new(inputs, outputs))))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::tests::*;
  use crate::common::to_input;

  #[test]
  fn test_parse_values() {
    let res = values(to_input(b"-1,2,  3  ,4"));
    assert_full_result(res, vec![-1, 2, 3, 4]);
  }

  #[test]
  fn test_parse_array_squares() {
    let res = array(to_input(b"[1, -2, 3]"));
    assert_full_result(res, vec![1, -2, 3]);
  }

  #[test]
  fn test_parse_array_simple() {
    let res = array(to_input(b"10,5,2"));
    assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_array_mixed() {
    let open_res = array(to_input(b"[10,5,2"));
    assert_cannot_parse(open_res);

    // Valid as an array with trailing ]
    let close_res = array(to_input(b"10,5,2]"));
    assert_cannot_parse(close_res);
  }

  #[test]
  fn test_parse_test_case() {
    let res = test_case(to_input(b"/// [1,2] -> [-1]  \nnext"));
    assert_result(res, TestCase::new(vec![1, 2], vec![-1]), to_input(b"next"));
  }

  #[test]
  fn test_parse_unclosed_test_case() {
    let res = test_case(to_input(b"/// [1,2 -> -1]  \nnext"));
    assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_opposite_test_case() {
    let res = test_case(to_input(b"/// 1,2] -> [-1  \nnext"));
    assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_opening_test_case() {
    let res = test_case(to_input(b"/// [1,2 -> [-1  \nnext"));
    assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_ending_test_case() {
    let res = test_case(to_input(b"/// 1,2] -> -1]  \nnext"));
    assert_cannot_parse(res);
  }

}
