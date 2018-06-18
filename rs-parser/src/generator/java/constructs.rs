use generator::java::constants::{Type, ArrayType};

#[derive(Debug, PartialEq, Clone)]
pub struct Signature {
  pub return_type: Type,
  pub parameter_types: Vec<Type>
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Operation {
  /// Load a value from an array
  // aaload,
  /// Push the value into an array at a given index
  // aastore,

  /// Loads a reference of a local variable into the stack
  /// 
  /// Structure
  /// ```
  ///  1. Index of the local variable
  /// ```
  aload(u8),
  /// Stores a reference into a local variable
  /// 
  /// Structure
  /// ```
  ///  1. Index of the local variable
  /// ```
  astore(u8),
  /// Pushes a byte into the operand stack
  /// 
  /// Structure
  /// ```
  ///  1. Value to push
  /// ```
  bipush(i8),
  /// Duplicates the top value of the operand stack
  dup,
  /// Store an integer into an array
  iastore,
  /// Push the constant 1 to the operand stack
  iconst_1,
  /// Invoke an instance method.
  /// 
  /// Special handling is provided for superclass, private, and instance
  /// initialization method invocations
  /// 
  /// Structure
  /// ```
  ///  1. Index of the method info for the method to call
  /// ```
  invokespecial(u16),
  /// Invoke an instance method, dispatching the call to the appropriate class.
  /// 
  /// Structure
  /// ```
  ///  1. Index of the method info for the method to call
  /// ```
  invokevirtual(u16),
  /// Invoke a static method from a class
  /// 
  /// Structure
  /// ```
  ///  1. Index of the method info for the method to call
  /// ```
  invokestatic(u16),
  ldc(u16),
  new(u16),
  newarray(ArrayType),
  return_void
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
  /// Code attribute
  /// Structure
  /// ```
  ///  1. max stack size
  ///  2. Operations
  /// ```
  Code(u16, Vec<Operation>)
}

#[derive(Debug)]
pub struct Method {
  pub access: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes: Vec<(u16, Attribute)>
}

pub fn merge_codes(mut codes: Vec<Attribute>) -> Attribute {
  codes.drain(0..)
    .fold(
      Attribute::Code(0, vec![]),
      |mut r, mut e| { 
      {
        let Attribute::Code(max, ref mut ops) = e;
        let &mut Attribute::Code(ref mut result_max, ref mut result_ops) = &mut r;
        *result_max += max;

        ops.drain(0..).fold(
          result_ops,
          |o, e| {
            o.push(e);
            o
          });
      }

      r
    })
}

#[cfg(test)]
mod tests {
  use super::*;
  use generator::java::constants::ArrayType;

  #[test]
  fn test_merge_codes() {
    let code1 = Attribute::Code(
      2,
      vec![
        Operation::bipush(12),
        Operation::dup
      ]
    );
    let code2 = Attribute::Code(
      3,
      vec![
        Operation::ldc(8),
        Operation::newarray(ArrayType::BOOLEAN)
      ]
    );
    let code3 = Attribute::Code(
      4,
      vec![
        Operation::invokespecial(4),
        Operation::invokestatic(5),
        Operation::return_void
      ]
    );
    let merged_code = merge_codes(vec![code1, code2, code3]);

    assert_eq!(
      merged_code,
      Attribute::Code(
        9,
        vec![
          Operation::bipush(12),
          Operation::dup,
          Operation::ldc(8),
          Operation::newarray(ArrayType::BOOLEAN),
          Operation::invokespecial(4),
          Operation::invokestatic(5),
          Operation::return_void
        ]
      ));
  }

}
