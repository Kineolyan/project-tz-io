pub mod address;
pub mod common;
pub mod instruction;
pub mod syntax;
pub mod test;

use nom;
use std::result::Result;

use nom::character::complete::space0;
use parser::common::{opt_eol, to_string};
use parser::syntax::{node_list, NodeBlock};
use parser::test::test_case;

pub use parser::test::TestCase;
pub struct ParsingTree {
  pub nodes: Vec<NodeBlock>,
  pub tests: Vec<TestCase>,
}
pub type ParsingResult = Result<ParsingTree, ()>;

pub fn program(
  input: &[u8],
) -> nom::IResult<&[u8], (Vec<NodeBlock>, Vec<TestCase>, Vec<TestCase>)> {
  // do_parse!(
  // 	opt_eol >>
  // 	start_cases: many0!(test_case) >>
  // 	many0!(do_parse!(space0 >> tag!("\n") >> ())) >>
  // 	list: node_list >>
  // 	opt_eol >>
  // 	end_cases: many0!(test_case) >>
  // 	opt_eol >>
  // 	(list, start_cases, end_cases)
  // )
  todo!("Migration in progress")
}

pub fn parse(input: &[u8]) -> ParsingResult {
  let res = program(input);
  match res {
    Ok((i, (list, mut start_cases, mut end_cases))) => {
      if i.len() == 0 {
        // Move all results to one list
        start_cases.append(&mut end_cases);

        let tree = ParsingTree {
          nodes: list,
          tests: start_cases,
        };
        Result::Ok(tree)
      } else {
        println!("Remaining unparsed content {}", to_string(i).unwrap());
        Result::Err(())
      }
    }
    Err(nom::Err::Error(ctx)) | Err(nom::Err::Failure(ctx)) => {
      let mut first = true;
      println!("{:?}", ctx);
      // FIXME
      // let errors = error_to_list(&ctx);
      // for error in &errors {
      //   if first {
      //     println!("Error while parsing: {:?}", error);
      //     first = false;
      //   } else {
      //     println!("  caused by: {:?}", error);
      //   }
      // }
      // println!("{:?}", e);
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

  use parser::address::{Node, Port};
  use parser::common::tests::*;
  use parser::instruction::Operation;
  use parser::instruction::ValuePointer;
  use parser::syntax::{InputMapping, OutputMapping};

  #[test]
  fn test_program_without_tests() {
    let content = b"// Start of the program
// Another comment

Node #1
==========
IN:1 -> 1
--
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

    assert_full_result(res, (nodes, vec![], vec![]));
  }

  #[test]
  fn test_program_with_tests() {
    let content = b"// Start of the program
// Another comment
/// [1, 2] -> [3]
/// [1, 2, 4] -> -8

Node #1
==========
IN:1 -> 1
---------
MOV <1,  >1
---------
1 -> #2:2
=======

/// 1 -> [-1, 1]
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
    let first_tests = vec![
      TestCase::new(vec![1, 2], vec![3]),
      TestCase::new(vec![1, 2, 4], vec![-8]),
    ];
    let last_tests = vec![TestCase::new(vec![1], vec![-1, 1])];

    assert_full_result(res, (nodes, first_tests, last_tests));
  }

  #[test]
  fn test_program_with_trailing_spaces() {
    let content = b"// Start of the program
// Another comment
/// [1, 2] -> [3]
/// [1, 2, 4] -> -8

Node #1
==========
MOV <1,  >1
=======

/// 1 -> [-1, 1]
// End comment, to conclude
   ";

    let res = program(content);
    let nodes = vec![(
      Node::new_node("1"),
      vec![],
      vec![],
      vec![Operation::MOV(ValuePointer::PORT(1), ValuePointer::PORT(1))],
    )];
    let first_tests = vec![
      TestCase::new(vec![1, 2], vec![3]),
      TestCase::new(vec![1, 2, 4], vec![-8]),
    ];
    let last_tests = vec![TestCase::new(vec![1], vec![-1, 1])];

    assert_result(res, (nodes, first_tests, last_tests), b"   ");
  }
}
