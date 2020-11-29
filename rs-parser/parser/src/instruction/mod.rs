mod base;
pub mod condition;
mod math;
mod memory;
mod mov;

use nom;

use crate::instruction::condition::*;
use crate::instruction::math::*;
use crate::instruction::memory::*;
use crate::instruction::mov::*;

pub fn parse_instruction(input: &[u8]) -> nom::IResult<&[u8], language::instruction::Operation> {
  nom::branch::alt((
    mov_operation,
    swp_operation,
    sav_operation,
    add_operation,
    sub_operation,
    neg_operation,
    // label_operation |
    jmp_operation,
    jez_operation,
    jnz_operation,
    jlz_operation,
    jgz_operation,
    jro_operation,
  ))(input)
}

