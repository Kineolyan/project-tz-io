use nom::IResult;
// use nom::number::complete::be_i8;
use nom::character::complete::space0;
use language::test::TestCase;

pub fn values(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
  // separated_nonempty_list_complete!(
  //   do_parse!(space0 >> tag!(",") >> space0 >> ()),
  //   be_i8
  // )
  todo!()
}

pub fn array(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
  // alt!(
  //   delimited!(
  //     tag!("["),
  //     values,
  //     tag!("]")
  //   ) |
  //   values
  // )
  todo!()
}

named!(pub test_case<&[u8], TestCase>,
  do_parse!(
    tag!("///") >> space0 >>
    ins: array >>
    space0 >> tag!("->") >> space0 >>
    outs: array >>
    space0 >> tag!("\n") >>
    (TestCase::new(ins, outs))
  )
);

#[cfg(test)]
mod tests {
  use super::*;
	use crate::common::to_input;
	use crate::common::tests::*;

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
    assert_full_result(res, vec![10, 5, 2]);
  }

  #[test]
  fn test_parse_array_mixed() {
    let open_res = array(to_input(b"[10,5,2"));
    assert_cannot_parse(open_res);

    // Valid as an array with trailing ]
    let close_res = array(to_input(b"10,5,2]"));
    assert_result(close_res, vec![10, 5, 2], to_input(b"]"));
  }

  #[test]
  fn test_parse_test_case() {
		let res = test_case(to_input(b"/// [1,2] -> [-1]  \nnext"));
		assert_result(res, TestCase::new(vec![1, 2], vec![-1]), to_input(b"next"));
  }

  #[test]
  fn test_parse_minimal_test_case() {
		let res = test_case(to_input(b"/// 1,2 -> -1  \nnext"));
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

  // #[test]
  // fn test_parse_test_cases() {
	// 	let res = test_cases(to_input(b"/// [1,2] -> [-1]  \n/// 3->3end"));
	// 	assert_result(
  //     res,
  //     vec![
  //       TestCase::new(vec![1, 2], vec![-1]),
  //       TestCase::new(vec![3], vec![3])
  //     ],
  //     to_input(b"end"));
  // }

}