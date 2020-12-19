use crate::java::constants::{ArrayType, Type};
use std::cmp;

#[derive(Debug, PartialEq, Clone)]
pub struct Signature {
    pub return_type: Type,
    pub parameter_types: Vec<Type>,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Operation {
    /// Load a value from an array
    // aaload,
    /// Push the value into an array at a given index
    // aastore,
    /// Push null onto the stack
    #[allow(dead_code)]
    aconst_null,
    /// Loads a reference of a local variable into the stack
    ///
    /// Structure
    /// ```md
    ///  1. Index of the local variable
    /// ```
    aload(u8),
    /// Return reference from method
    areturn,
    /// Stores a reference into a local variable
    ///
    /// Structure
    /// ```md
    ///  1. Index of the local variable
    /// ```
    astore(u8),
    /// Pushes a byte into the operand stack
    ///
    /// Structure
    /// ```md
    ///  1. Value to push
    /// ```
    bipush(i8),
    /// Duplicates the top value of the operand stack
    dup,
    /// Store an integer into an array
    iastore,
    /// Push the constant 1 to the operand stack
    iconst_1,
    /// Push the constant -1 to the operand stack
    iconst_m1,
    /// Invoke an instance method.
    ///
    /// Special handling is provided for superclass, private, and instance
    /// initialization method invocations
    ///
    /// Structure
    /// ```md
    ///  1. Index of the method info for the method to call
    /// ```
    invokespecial(u16),
    /// Invoke an instance method, dispatching the call to the appropriate class.
    ///
    /// Structure
    /// ```md
    ///  1. Index of the method info for the method to call
    /// ```
    invokevirtual(u16),
    /// Invoke a static method from a class
    ///
    /// Structure
    /// ```md
    ///  1. Index of the method info for the method to call
    /// ```
    invokestatic(u16),
    /// Invoke a dynamic method from a class
    ///
    /// Structure
    /// ```md
    ///  1. Index of the method info for the method to call
    /// ```
    #[allow(dead_code)]
    invokedynamic(u16),
    /// Invoke interface method from a class
    ///
    /// Structure
    /// ```md
    ///  1. Index of the method info for the method to call
    ///  2. Arg count of the method
    /// ```
    invokeinterface(u16, u8),
    ldc(u16),
    new(u16),
    newarray(ArrayType),
    /// Do nothing
    nop,
    return_void,
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    /// Code attribute
    /// Structure
    /// ```md
    ///  - max_stack: max stack size
    ///  - operations: Operations
    ///  - locals: number of local vars
    /// ```
    Code {
        max_stack: u16,
        operations: Vec<Operation>,
        locals: u16,
    },
}

#[derive(Debug)]
pub struct Method {
    pub access: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<(u16, Attribute)>,
}

pub fn count_local_vars(signature: Option<&Signature>, operations: &[Operation]) -> u16 {
    let arg_count = signature
        .map(|s| s.parameter_types.len() as u16)
        .unwrap_or(0);
    let op_count = operations
        .iter()
        .map(|op| match op {
            Operation::aload(ref idx) => *idx as u16 + 1,
            Operation::astore(ref idx) => *idx as u16 + 1,
            _ => 0u16,
        })
        .max()
        .unwrap_or(0u16);

    cmp::max(op_count, arg_count)
}

pub fn merge_codes(opt_signature: Option<&Signature>, mut codes: Vec<Attribute>) -> Attribute {
    let initial_locals: u16 = opt_signature
        .map(|s| s.parameter_types.len() as u16)
        .unwrap_or(0);
    codes.drain(0..).fold(
        Attribute::Code {
            max_stack: 0,
            operations: vec![],
            locals: initial_locals,
        },
        |mut r, mut e| {
            {
                let Attribute::Code {
                    max_stack: max,
                    operations: ref mut ops,
                    locals,
                } = e;
                let &mut Attribute::Code {
                    max_stack: ref mut result_max,
                    operations: ref mut result_ops,
                    locals: ref mut result_locals,
                } = &mut r;
                *result_max += max;
                *result_locals = cmp::max(locals, *result_locals);

                ops.drain(0..).fold(result_ops, |o, e| {
                    o.push(e);
                    o
                });
            }

            r
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::constants::{ArrayType, Type};

    #[test]
    fn test_count_local_vars_from_loads() {
        let count = count_local_vars(
            None,
            &[
                Operation::bipush(5),
                Operation::aload(1),
                Operation::aload(4),
                Operation::dup,
                Operation::aload(2),
                Operation::ldc(52),
            ],
        );
        assert_eq!(count, 5);
    }

    #[test]
    fn test_count_local_vars_from_stores() {
        let count = count_local_vars(
            None,
            &[
                Operation::iastore,
                Operation::astore(10),
                Operation::astore(3),
                Operation::new(52),
                Operation::astore(2),
            ],
        );
        assert_eq!(count, 11);
    }

    #[test]
    fn test_count_local_vars() {
        let count = count_local_vars(None, &[Operation::aload(3), Operation::astore(7)]);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_empty_count_local_vars() {
        let count = count_local_vars(None, &[]);
        assert_eq!(count, 0);
    }

    #[test]
    fn count_local_vars_with_signature() {
        let count1 = count_local_vars(
            Some(&Signature {
                return_type: Type::Boolean,
                parameter_types: vec![Type::Integer, Type::Integer, Type::Boolean],
            }),
            &[Operation::aload(1)],
        );
        assert_eq!(count1, 3);

        let count1 = count_local_vars(
            Some(&Signature {
                return_type: Type::Void,
                parameter_types: vec![Type::Boolean],
            }),
            &[Operation::astore(4)],
        );
        assert_eq!(count1, 5);
    }

    #[test]
    fn count_local_vars_with_empty_signature() {
        let count = count_local_vars(
            Some(&Signature {
                return_type: Type::Boolean,
                parameter_types: vec![],
            }),
            &[],
        );
        assert_eq!(count, 0);
    }

    #[test]
    fn test_merge_codes() {
        let code1 = Attribute::Code {
            max_stack: 2,
            operations: vec![Operation::bipush(12), Operation::dup],
            locals: 3,
        };
        let code2 = Attribute::Code {
            max_stack: 3,
            operations: vec![Operation::ldc(8), Operation::newarray(ArrayType::BOOLEAN)],
            locals: 2,
        };
        let code3 = Attribute::Code {
            max_stack: 4,
            operations: vec![
                Operation::invokespecial(4),
                Operation::invokestatic(5),
                Operation::return_void,
            ],
            locals: 0,
        };
        let merged_code = merge_codes(None, vec![code1, code2, code3]);

        assert_eq!(
            merged_code,
            Attribute::Code {
                max_stack: 9,
                operations: vec![
                    Operation::bipush(12),
                    Operation::dup,
                    Operation::ldc(8),
                    Operation::newarray(ArrayType::BOOLEAN),
                    Operation::invokespecial(4),
                    Operation::invokestatic(5),
                    Operation::return_void
                ],
                locals: 3
            }
        );
    }

    #[test]
    fn test_merge_codes_with_signature() {
        let code1 = Attribute::Code {
            max_stack: 2,
            operations: vec![Operation::bipush(12), Operation::dup],
            locals: 1,
        };
        let code2 = Attribute::Code {
            max_stack: 3,
            operations: vec![Operation::ldc(8), Operation::newarray(ArrayType::BOOLEAN)],
            locals: 1,
        };
        let code3 = Attribute::Code {
            max_stack: 4,
            operations: vec![
                Operation::invokespecial(4),
                Operation::invokestatic(5),
                Operation::return_void,
            ],
            locals: 5,
        };
        let Attribute::Code {
            locals: locals1,
            max_stack: _,
            operations: _,
        } = merge_codes(
            Some(&Signature {
                return_type: Type::Void,
                parameter_types: vec![Type::Integer, Type::Integer, Type::Integer],
            }),
            vec![code1, code2],
        );
        assert_eq!(locals1, 3);

        let Attribute::Code {
            locals: locals2,
            max_stack: _,
            operations: _,
        } = merge_codes(
            Some(&Signature {
                return_type: Type::Void,
                parameter_types: vec![Type::Integer, Type::Integer, Type::Integer],
            }),
            vec![code3],
        );
        assert_eq!(locals2, 5);
    }
}
