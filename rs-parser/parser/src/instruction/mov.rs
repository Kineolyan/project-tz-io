use nom::branch;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::IResult;

use crate::instruction::base as ptr;
use language::instruction::Operation;

fn consume_mov(input: &[u8]) -> IResult<&[u8], ()> {
  let (rest, _) = nom::sequence::tuple((tag("MOV"), space1))(input)?;
  Ok((rest, ()))
}

fn mov_from_in(input: &[u8]) -> IResult<&[u8], Operation> {
  let (rest, (from, _, _, _, to)) = nom::sequence::tuple((
    ptr::input_pointer,
    space0,
    tag(","),
    space0,
    branch::alt((ptr::acc_pointer, ptr::nil_pointer, ptr::output_pointer)),
  ))(input)?;
  Ok((rest, Operation::MOV(from, to)))
}

fn mov_to_out(input: &[u8]) -> IResult<&[u8], Operation> {
  let (rest, (from, _, _, _, to)) = nom::sequence::tuple((
    branch::alt((ptr::acc_pointer, ptr::nil_pointer, ptr::value_pointer)),
    space0,
    tag(","),
    space0,
    ptr::output_pointer,
  ))(input)?;
  Ok((rest, Operation::MOV(from, to)))
}

fn mov_accs(input: &[u8]) -> IResult<&[u8], Operation> {
  let (rest, (from, _, _, _, to)) = nom::sequence::tuple((
    branch::alt((ptr::value_pointer, ptr::acc_pointer, ptr::nil_pointer)),
    space0,
    tag(","),
    space0,
    ptr::acc_pointer,
  ))(input)?;
  Ok((rest, Operation::MOV(from, to)))
}

pub fn mov_operation(input: &[u8]) -> IResult<&[u8], Operation> {
  let (input, _) = consume_mov(input)?;
  nom::branch::alt((mov_from_in, mov_to_out, mov_accs))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::common::tests::*;
  use crate::common::to_input;
  use language::instruction::ValuePointer;

  #[test]
  fn test_parse_mov_in_to_out() {
    let res = mov_operation(to_input(b"MOV <1, >2"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(2)),
    );
  }

  #[test]
  fn test_parse_mov_in_to_acc() {
    let res = mov_operation(to_input(b"MOV <1, ACC"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
    );
  }

  #[test]
  fn test_parse_mov_value_to_out() {
    let res = mov_operation(to_input(b"MOV 12, >3"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::VALUE(12), ValuePointer::PORT(3)),
    );
  }

  #[test]
  fn test_parse_mov_acc_to_out() {
    let res = mov_operation(to_input(b"MOV ACC, >4"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(4)),
    );
  }

  #[test]
  fn test_parse_mov_value_to_acc() {
    let res = mov_operation(to_input(b"MOV 45, ACC"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::VALUE(45), ValuePointer::ACC),
    );
  }

  #[test]
  fn test_parse_mov_val_to_acc() {
    let res = mov_operation(to_input(b"MOV 76, ACC"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::VALUE(76), ValuePointer::ACC),
    );
  }

  #[test]
  fn test_parse_mov_acc_to_acc() {
    let res = mov_operation(to_input(b"MOV ACC, ACC"));
    assert_full_result(res, Operation::MOV(ValuePointer::ACC, ValuePointer::ACC));
  }

  #[test]
  fn test_parse_mov_nil_to_acc() {
    let res = mov_operation(to_input(b"MOV NIL, ACC"));
    assert_full_result(res, Operation::MOV(ValuePointer::NIL, ValuePointer::ACC));
  }

  #[test]
  fn test_parse_mov_nil_to_out() {
    let res = mov_operation(to_input(b"MOV NIL, >12"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::NIL, ValuePointer::PORT(12)),
    );
  }

  #[test]
  fn test_parse_mov_in_to_nil() {
    let res = mov_operation(to_input(b"MOV <1, NIL"));
    assert_full_result(
      res,
      Operation::MOV(ValuePointer::PORT(1), ValuePointer::NIL),
    );
  }

  #[test]
  fn test_cannot_parse_out_to_in() {
    let res = mov_operation(to_input(b"MOV >1, <2"));
    assert_cannot_parse(res);
  }
}
