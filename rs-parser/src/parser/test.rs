use nom::{space, newline};

use parser::common::{RawData, be_i8, ospace};

named!(values<&RawData, Vec<i8> >,
  separated_nonempty_list_complete!(
    do_parse!(ospace >> tag!(",") >> ospace >> ()),
    be_i8
  )
);

named!(array<&RawData, Vec<i8> >,
  alt!(
    do_parse!(
      tag!("[") >> ospace >>
      vs: values >>
      ospace >> tag!("]") >>
      (vs)
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

named!(pub test_case<&RawData, (TestCase)>,
  do_parse!(
    tag!("///") >> ospace >>
    ins: array >> 
    ospace >> tag!("->") >> ospace >>
    outs: array >> 
    ospace >>
    (TestCase::new(ins, outs))
  )
);

named!(pub test_cases<&RawData, Vec<TestCase> >,
  separated_nonempty_list_complete!(
    newline,
    test_case
  )
);

#[cfg(test)]
mod tests {
  use super::*;
	use parser::common::tests::{assert_result, assert_full_result, assert_cannot_parse};

  #[test]
  fn test_parse_values() {
    let res = values(b"-1,2,  3  ,4");
    assert_full_result(res, vec![-1, 2, 3, 4]);
  }

  #[test]
  fn test_parse_array_squares() {
    let res = array(b"[1, -2, 3]");
    assert_full_result(res, vec![1, -2, 3]);
  }

  #[test]
  fn test_parse_array_simple() {
    let res = array(b"10,5,2");
    assert_full_result(res, vec![10, 5, 2]);
  }

  #[test]
  fn test_parse_array_mixed() {
    let open_res = array(b"[10,5,2");
    assert_cannot_parse(open_res);
    let close_res = array(b"10,5,2]");
    assert_cannot_parse(close_res);
  }

  #[test]
  fn test_parse_test_case() {
		let res = test_case(b"/// [1,2] -> [-1]  next");
		assert_result(res, TestCase::new(vec![1, 2], vec![-1]), b"next");
  }

  #[test]
  fn test_parse_test_cases() {
		let res = test_cases(b"/// [1,2] -> [-1]  \n/// 3->3end");
		assert_result(
      res, 
      vec![
        TestCase::new(vec![1, 2], vec![-1]),
        TestCase::new(vec![3], vec![3])
      ],
      b"end");
  }

}