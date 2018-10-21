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

named!(pub test_case<&RawData, (Vec<i8>, Vec<i8>)>,
  do_parse!(
    tag!("///") >> ospace >>
    ins: array >> 
    ospace >> tag!("->") >> ospace >>
    outs: array >> 
    ospace >> tag!("\n") >>
    (vec![], vec![])
  )
);

#[cfg(test)]
mod tests {
  use super::*;
	use parser::common::tests::{assert_result};

  #[test]
  fn test_parse_test_case() {
		let res = test_case(b"/// [1,2] -> [-1]  \nnext");
		assert_result(res, (vec![], vec![]), b"\nnext");
  }

}