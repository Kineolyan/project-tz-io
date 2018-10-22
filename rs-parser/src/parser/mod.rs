pub mod address;
pub mod syntax;
pub mod common;
pub mod instruction;
pub mod test;

use nom::{IResult, error_to_list, newline};
use std::result::Result;
use std::str::from_utf8;

use parser::common::{RawData, opt_eol};
use parser::test::{TestCase, test_cases};
use parser::syntax::{NodeBlock, node_list};

pub struct ParsingTree {
  pub nodes: Vec<NodeBlock>,
  pub tests: Vec<TestCase>
}
pub type ParsingResult = Result<ParsingTree, ()>;

named!(pub program<&RawData, (Vec<NodeBlock>, Vec<TestCase>, Vec<TestCase>)>,
	do_parse!(
		opt_eol >>
		start_cases: test_cases >> 
		many1!(newline) >>
		list: node_list >>
		opt_eol >>
		end_cases: test_cases >>
		opt_eol >>
		(list, start_cases, end_cases)
	)
);

pub fn parse(input: &common::RawData) -> ParsingResult {
  let mut res = program(input);
  match res {
    IResult::Done(i, (list, start_cases, end_cases)) => {
      if i.len() == 0 {
        // Move all results to one list
        start_cases.append(&mut end_cases);

        let tree = ParsingTree { nodes: list, tests: start_cases};
        Result::Ok(tree)
      } else {
        println!("Remaining unparsed content {}", from_utf8(i).unwrap());
        Result::Err(())
      }
    },
    IResult::Error(e) => {
      let mut first = true;
      println!("{:?}", e);
      let errors = error_to_list(&e);
      for error in &errors {
        if first {
          println!("Error while parsing: {:?}", error);
          first = false;
        } else {
          println!("  caused by: {:?}", error);
        }
      }
      // println!("{:?}", e);
      Result::Err(())
    },
    IResult::Incomplete(needed) => {
      println!("Missing data. Needed {:?}", needed);
      Result::Err(())
    }
  }
}