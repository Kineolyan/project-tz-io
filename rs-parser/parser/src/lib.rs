// #![cfg(feature = "alloc")]

extern crate nom;

mod address;
mod common;
mod instruction;
mod mapping;
mod syntax;
mod test;

use std::result::Result;

use language::syntax::NodeBlock;
use language::syntax::Program;
use language::test::TestCase;

pub type ParsingResult = Result<Program, ()>;

fn program(input: &[u8]) -> nom::IResult<&[u8], (Vec<NodeBlock>, Option<TestCase>)> {
    use crate::common::opt_eol;

    let (input, _) = opt_eol(input)?;
    let (input, test_case) = nom::combinator::opt(crate::test::test_case)(input)?;
    let (input, _) = opt_eol(input)?;
    let (input, nodes) = crate::syntax::node_list(input)?;
    let (input, _) = opt_eol(input)?;

    Ok((input, (nodes, test_case)))
}

fn print_error(e: &nom::error::Error<&[u8]>) {
    println!(
        "Error = {:#?}\n{:#?}",
        e.code,
        crate::common::to_string(e.input).expect("Cannot display content")
    )
}

pub fn parse(input: &[u8]) -> ParsingResult {
    let res = program(input);
    match res {
        Ok((i, (list, test_cases))) => {
            if i.is_empty() {
                let tree = Program {
                    nodes: list,
                    tests: test_cases,
                };
                Result::Ok(tree)
            } else {
                println!(
                    "Remaining unparsed content {}",
                    crate::common::to_string(i).unwrap()
                );
                Result::Err(())
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            // if cfg!(feature = "alloc") {
            //     nom::error::convert_error(input, e);
            // }
            print_error(&e);
            Result::Err(())
        }
        Err(nom::Err::Incomplete(needed)) => {
            println!("Missing data. Needed {:?}", needed);
            Result::Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use common::tests::*;
    use language::address::{Node, Port};
    use language::instruction::{Operation, ValuePointer};
    use language::syntax::{InputMapping, OutputMapping};

    #[test]
    fn test_program_without_tests() {
        let content = b"// Start of the program
// Another comment

Node #1
==========
IN:1 -> 1
-------
MOV <1,  >1
---------
1 -> #2:2
=======

 Node #2
==========
#1:1 -> 2
----------
MOV <2, >2
----------
2 -> #3:3
==========

// End comment, to conclude
";

        let res = program(content);
        let nodes = vec![
            (
                Node::new_node("1"),
                vec![InputMapping {
                    from: Port::new(Node::In, 1.into()),
                    to: 1.into(),
                }],
                vec![OutputMapping {
                    from: 1.into(),
                    to: Port::named_port(&"2", 2.into()),
                }],
                vec![Operation::MOV(
                    ValuePointer::INPUT(1.into()),
                    ValuePointer::OUTPUT(1.into()),
                )],
            ),
            (
                Node::new_node("2"),
                vec![InputMapping {
                    from: Port::named_port(&"1", 1.into()),
                    to: 2.into(),
                }],
                vec![OutputMapping {
                    from: 2.into(),
                    to: Port::named_port(&"3", 3.into()),
                }],
                vec![Operation::MOV(
                    ValuePointer::INPUT(2.into()),
                    ValuePointer::OUTPUT(2.into()),
                )],
            ),
        ];

        assert_full_result(res, (nodes, None));
    }

    #[test]
    fn test_program_with_tests() {
        let content = b"// Start of the program
// Another comment
/>> 1: [1 2]
/>> 2: [2 4]
/<< 1: [-1 -2]

Node #1
==========
IN:1 -> 1
---------
MOV <1,  >1
---------
1 -> #2:2
=======

// End comment, to conclude
";

        let res = program(content);
        let nodes = vec![(
            Node::new_node("1"),
            vec![InputMapping {
                from: Port::new(Node::In, 1.into()),
                to: 1.into(),
            }],
            vec![OutputMapping {
                from: 1.into(),
                to: Port::named_port(&"2", 2.into()),
            }],
            vec![Operation::MOV(
                ValuePointer::INPUT(1.into()),
                ValuePointer::OUTPUT(1.into()),
            )],
        )];
        let test_cases = Some(
            TestCase::default()
                .input_into(1.into(), vec![1, 2])
                .input_into(2.into(), vec![2, 4])
                .output_from(1.into(), vec![-1, -2]),
        );

        assert_full_result(res, (nodes, test_cases));
    }

    #[test]
    fn test_program_with_trailing_spaces() {
        let content = b"// Start of the program
Node #1
==========
MOV <1,  >1
=======
// End comment, to conclude
   ";

        let res = program(content);
        let nodes = vec![(
            Node::new_node("1"),
            vec![],
            vec![],
            vec![Operation::MOV(
                ValuePointer::INPUT(1.into()),
                ValuePointer::OUTPUT(1.into()),
            )],
        )];
        assert_result(res, (nodes, None), b"   ");
    }
}
