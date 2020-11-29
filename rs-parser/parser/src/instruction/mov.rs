use nom::IResult;
// use nom::character::complete::space0;

use language::instruction::Operation;
// use crate::instruction::base::*;

fn mov_from_in(input: &[u8]) -> IResult<&[u8], Operation> {
  todo!()
  // do_parse!(
  //   tag!("MOV") >> space >>
  //   from: input_pointer >>
  //   space0 >> tag!(",") >> space0 >>
  //   to: alt!(acc_pointer | nil_pointer | output_pointer) >>
  //   (Operation::MOV(from, to))
  // )
}

fn mov_to_out(input: &[u8]) -> IResult<&[u8], Operation> {
  // do_parse!(
  //   tag!("MOV") >> space >>
  //   from: alt!(acc_pointer | nil_pointer | value_pointer) >>
  //   space0 >> tag!(",") >> space0 >>
  //   to: output_pointer >>
  //   (Operation::MOV(from, to))
  // )
  todo!()
}

fn mov_accs(input: &[u8]) -> IResult<&[u8], Operation> {
  // do_parse!(
  //   tag!("MOV") >> space >>
  //   from: alt!(value_pointer | acc_pointer | nil_pointer ) >>
  //   space0 >> tag!(",") >> space0 >>
  //   to: acc_pointer >>
  //   (Operation::MOV(from, to))
  // )
  todo!()
}

pub fn mov_operation(input: &[u8]) -> IResult<&[u8], Operation> {
  nom::branch::alt((mov_from_in, mov_to_out, mov_accs))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

	use crate::common::to_input;
  use crate::common::tests::*;
  use language::instruction::ValuePointer;

  #[test]
  fn test_parse_mov_in_to_out() {
    let res = mov_operation(to_input(b"MOV <1, >2"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::PORT(2)
      )
    );
  }

  #[test]
  fn test_parse_mov_in_to_acc() {
    let res = mov_operation(to_input(b"MOV <1, ACC"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_value_to_out() {
    let res = mov_operation(to_input(b"MOV 12, >3"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(12),
        ValuePointer::PORT(3)
      )
    );
  }

  #[test]
  fn test_parse_mov_acc_to_out() {
    let res = mov_operation(to_input(b"MOV ACC, >4"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC,
        ValuePointer::PORT(4)
      )
    );
  }

  #[test]
  fn test_parse_mov_value_to_acc() {
    let res = mov_operation(to_input(b"MOV 45, ACC"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(45),
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_val_to_acc() {
    let res = mov_operation(to_input(b"MOV 76, ACC"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(76),
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_acc_to_acc() {
    let res = mov_operation(to_input(b"MOV ACC, ACC"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC,
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_nil_to_acc() {
    let res = mov_operation(to_input(b"MOV NIL, ACC"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::NIL,
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_nil_to_out() {
    let res = mov_operation(to_input(b"MOV NIL, >12"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::NIL,
        ValuePointer::PORT(12)
      )
    );
  }

  #[test]
  fn test_parse_mov_in_to_nil() {
    let res = mov_operation(to_input(b"MOV <1, NIL"));
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::NIL
      )
    );
  }

  #[test]
  fn test_cannot_parse_out_to_in() {
    let res = mov_operation(to_input(b"MOV >1, <2"));
    assert_cannot_parse(res);
  }
}
