mod base;
mod mov;
mod memory;
mod math;
pub mod condition;

use std::fmt;

use parser::common::Input;
use parser::instruction::mov::*;
use parser::instruction::memory::*;
use parser::instruction::math::*;
use parser::instruction::condition::*;

#[derive(PartialEq)]
pub enum ValuePointer {
  VALUE(u32),
  ACC,
  NIL,
  PORT(u32)
}

// The idea is to have ACC is the top of the stack, for ADD, SUB, NEG, ...
// and have multiple BAK if needed
#[derive(PartialEq)]
pub enum MemoryPointer {
	BAK(u8) // Limiting to 256 values
}

#[derive(PartialEq)]
pub enum Operation {
  MOV(ValuePointer, ValuePointer),
	SAV(MemoryPointer),
	SWP(MemoryPointer),
  ADD(ValuePointer),
  SUB(ValuePointer),
  NEG,
  LABEL(String),
  JMP(String),
  JEZ(String),
  JNZ(String),
  JLZ(String),
  JGZ(String),
  JRO(ValuePointer)
}

named!(pub parse_instruction<Input, Operation>,
  alt!(
    mov_operation |
    swp_operation |
    sav_operation |
    add_operation |
    sub_operation |
    neg_operation |
    // label_operation |
    jmp_operation |
    jez_operation |
    jnz_operation |
    jlz_operation |
    jgz_operation |
    jro_operation
  )
);

// Default implementations for printing

impl fmt::Debug for ValuePointer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl fmt::Display for ValuePointer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl ValuePointer {
	fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ValuePointer::VALUE(ref value) => write!(f, "Value({})", value),
      &ValuePointer::ACC => write!(f, "ACC"),
      &ValuePointer::NIL => write!(f, "NIL"),
      &ValuePointer::PORT(ref port) => write!(f, "Port({})", port)
    }
  }
}

impl fmt::Debug for MemoryPointer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl fmt::Display for MemoryPointer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl MemoryPointer {
	fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &MemoryPointer::BAK(ref slot) => write!(f, "BAK({})", slot)
    }
  }
}

impl fmt::Debug for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.do_fmt(f)
  }
}

impl Operation {
	fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Operation::MOV(ref from, ref to) => write!(f, "MOV {} {}", from, to),
      &Operation::SAV(ref ptr) => write!(f, "SAV {}", ptr),
      &Operation::SWP(ref ptr) => write!(f, "SWP {}", ptr),
      &Operation::ADD(ref ptr) => write!(f, "ADD {}", ptr),
      &Operation::SUB(ref ptr) => write!(f, "SUB {}", ptr),
      &Operation::NEG => write!(f, "NEG"),
      &Operation::LABEL(ref label) => write!(f, "LABEL {}", label),
      &Operation::JMP(ref label) => write!(f, "JMP {}", label),
      &Operation::JEZ(ref label) => write!(f, "JEZ {}", label),
      &Operation::JNZ(ref label) => write!(f, "JNZ {}", label),
      &Operation::JLZ(ref label) => write!(f, "JLZ {}", label),
      &Operation::JGZ(ref label) => write!(f, "JGZ {}", label),
      &Operation::JRO(ref ptr) => write!(f, "JRO {}", ptr)
    }
  }
}