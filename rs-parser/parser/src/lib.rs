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

fn program(input: &[u8]) -> nom::IResult<&[u8], (Vec<NodeBlock>, Vec<TestCase>)> {
    use crate::common::opt_eol;

    let (input, _) = opt_eol(input)?;
    let (input, mut start_cases) = nom::multi::many0(crate::test::test_case)(input)?;
    let (input, _) = opt_eol(input)?;
    let (input, nodes) = crate::syntax::node_list(input)?;
    let (input, _) = opt_eol(input)?;
    let (input, mut end_cases) = nom::multi::many0(crate::test::test_case)(input)?;
    let (input, _) = opt_eol(input)?;

    let test_cases = {
        let mut all = vec![];
        all.append(&mut start_cases);
        all.append(&mut end_cases);
        all
    };

    Ok((input, (nodes, test_cases)))
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
                    from: Port::new(Node::In, 1),
                    to: 1,
                }],
                vec![OutputMapping {
                    from: 1,
                    to: Port::named_port(&"2", 2),
                }],
                vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
            ),
            (
                Node::new_node("2"),
                vec![InputMapping {
                    from: Port::named_port(&"1", 1),
                    to: 2,
                }],
                vec![OutputMapping {
                    from: 2,
                    to: Port::named_port(&"3", 3),
                }],
                vec![Operation::MOV(ValuePointer::PORT(2), ValuePointer::PORT(2))],
            ),
        ];

        assert_full_result(res, (nodes, vec![]));
    }

    #[test]
    fn test_program_with_tests() {
        let content = b"// Start of the program
// Another comment
/>> [1, 2] -> [3]
/>> [1, 2, 4] -> [-8]

Node #1
==========
IN:1 -> 1
---------
MOV <1,  >1
---------
1 -> #2:2
=======

/>> [1] -> [-1, 1]
// End comment, to conclude
";

        let res = program(content);
        let nodes = vec![(
            Node::new_node("1"),
            vec![InputMapping {
                from: Port::new(Node::In, 1),
                to: 1,
            }],
            vec![OutputMapping {
                from: 1,
                to: Port::named_port(&"2", 2),
            }],
            vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
        )];
        let test_cases = vec![
            TestCase::new(vec![1, 2], vec![3]),
            TestCase::new(vec![1, 2, 4], vec![-8]),
            TestCase::new(vec![1], vec![-1, 1]),
        ];

        assert_full_result(res, (nodes, test_cases));
    }

    #[test]
    fn test_program_with_trailing_spaces() {
        let content = b"// Start of the program
// Another comment
/>> [1, 2] -> [3]
/>> [1, 2, 4] -> [-8]

Node #1
==========
MOV <1,  >1
=======

/>> [1] -> [-1, 1]
// End comment, to conclude
   ";

        let res = program(content);
        let nodes = vec![(
            Node::new_node("1"),
            vec![],
            vec![],
            vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
        )];
        let test_cases = vec![
            TestCase::new(vec![1, 2], vec![3]),
            TestCase::new(vec![1, 2, 4], vec![-8]),
            TestCase::new(vec![1], vec![-1, 1]),
        ];

        assert_result(res, (nodes, test_cases), b"   ");
    }
}
