use nom::number::complete::be_u32;
use crate::instruction::{ValuePointer, MemoryPointer};

named!(pub acc_pointer<&[u8], ValuePointer>,
	value!(ValuePointer::ACC, tag!("ACC"))
);

named!(pub nil_pointer<&[u8], ValuePointer>,
	value!(ValuePointer::NIL, tag!("NIL"))
);

named!(pub input_pointer<&[u8], ValuePointer>,
  do_parse!(
    tag!("<") >>
    port: be_u32 >>
    (ValuePointer::PORT(port))
  )
);

named!(pub output_pointer<&[u8], ValuePointer>,
  do_parse!(
    tag!(">") >>
    port: be_u32 >>
    (ValuePointer::PORT(port))
  )
);

named!(pub value_pointer<&[u8], ValuePointer>,
  map!(be_u32, |value| ValuePointer::VALUE(value))
);

named!(pub bak_pointer<&[u8], MemoryPointer>,
	value!(MemoryPointer::BAK(1), tag!("BAK"))
);

#[cfg(test)]
mod tests {
  use super::*;
	use crate::common::to_input;
  use crate::common::tests::*;

  #[test]
  fn test_parse_acc_pointer() {
    let res_explicit = acc_pointer(to_input(b"ACC"));
    assert_full_result(res_explicit, ValuePointer::ACC);
  }

  #[test]
  fn test_parse_nil_pointer() {
    let res_explicit = nil_pointer(to_input(b"NIL"));
    assert_full_result(res_explicit, ValuePointer::NIL);
  }

  #[test]
  fn test_parse_input_pointer() {
    let res = input_pointer(to_input(b"<12"));
    assert_full_result(res, ValuePointer::PORT(12));
  }

  #[test]
  fn test_parse_output_pointer() {
    let res = output_pointer(to_input(b">43"));
    assert_full_result(res, ValuePointer::PORT(43));
  }

  #[test]
  fn test_parse_value_pointer() {
    let res = value_pointer(to_input(b"37"));
    assert_full_result(res, ValuePointer::VALUE(37u32));
  }

	#[test]
	fn test_parse_bak_pointer() {
		let res = bak_pointer(to_input(b"BAK"));
		assert_full_result(res, MemoryPointer::BAK(1));
	}
}
