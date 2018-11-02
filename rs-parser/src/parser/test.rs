use parser::common::{Input, be_i8, ospace};

named!(values<Input, Vec<i8> >,
  separated_nonempty_list_complete!(
    do_parse!(ospace >> tag!(",") >> ospace >> ()),
    be_i8
  )
);

named!(array<Input, Vec<i8> >,
  alt!(
    delimited!(
      tag!("["),
      values,
      tag!("]")
    ) |
    values
  )
);

#[derive(Debug, PartialEq)]
pub struct TestCase {
  pub ins: Vec<i8>,
  pub outs: Vec<i8>
}

impl TestCase {
  pub fn new(ins: Vec<i8>, outs: Vec<i8>) -> TestCase {
    TestCase { ins: ins, outs: outs }
  }
}

named!(pub test_case<Input, TestCase>,
  do_parse!(
    tag!("///") >> ospace >>
    ins: array >> 
    ospace >> tag!("->") >> ospace >>
    outs: array >> 
    ospace >> tag!("\n") >>
    (TestCase::new(ins, outs))
  )
);

#[cfg(test)]
mod tests {
  use super::*;
	use parser::common::tests::*;

  #[test]
  fn test_parse_values() {
    let res = values(input(b"-1,2,  3  ,4"));
    assert_full_result(res, vec![-1, 2, 3, 4]);
  }

  #[test]
  fn test_parse_array_squares() {
    let res = array(input(b"[1, -2, 3]"));
    assert_full_result(res, vec![1, -2, 3]);
  }

  #[test]
  fn test_parse_array_simple() {
    let res = array(input(b"10,5,2"));
    assert_full_result(res, vec![10, 5, 2]);
  }

  #[test]
  fn test_parse_array_mixed() {
    let open_res = array(input(b"[10,5,2"));
    assert_incomplete(open_res);

    // Valid as an array with trailing ]
    let close_res = array(input(b"10,5,2]"));
    assert_result(close_res, vec![10, 5, 2], input(b"]"));
  }

  #[test]
  fn test_parse_test_case() {
		let res = test_case(input(b"/// [1,2] -> [-1]  \nnext"));
		assert_result(res, TestCase::new(vec![1, 2], vec![-1]), input(b"next"));
  }

  #[test]
  fn test_parse_minimal_test_case() {
		let res = test_case(input(b"/// 1,2 -> -1  \nnext"));
		assert_result(res, TestCase::new(vec![1, 2], vec![-1]), input(b"next"));
  }

  #[test]
  fn test_parse_unclosed_test_case() {
		let res = test_case(input(b"/// [1,2 -> -1]  \nnext"));
		assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_opposite_test_case() {
		let res = test_case(input(b"/// 1,2] -> [-1  \nnext"));
		assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_opening_test_case() {
		let res = test_case(input(b"/// [1,2 -> [-1  \nnext"));
		assert_cannot_parse(res);
  }

  #[test]
  fn test_parse_ending_test_case() {
		let res = test_case(input(b"/// 1,2] -> -1]  \nnext"));
		assert_cannot_parse(res);
  }

  // #[test]
  // fn test_parse_test_cases() {
	// 	let res = test_cases(input(b"/// [1,2] -> [-1]  \n/// 3->3end"));
	// 	assert_result(
  //     res, 
  //     vec![
  //       TestCase::new(vec![1, 2], vec![-1]),
  //       TestCase::new(vec![3], vec![3])
  //     ],
  //     input(b"end"));
  // }

}