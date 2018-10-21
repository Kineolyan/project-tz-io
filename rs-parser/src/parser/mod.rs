pub mod address;
pub mod syntax;
pub mod common;
pub mod instruction;
pub mod test;

use nom::{IResult, error_to_list};
use std::result::Result;
use std::str::from_utf8;

pub type ParsingTree = Vec<syntax::NodeBlock>;
pub type ParsingResult = Result<ParsingTree, ()>;

pub fn parse(input: &common::RawData) -> ParsingResult {
  let res = syntax::node_list(input);
  match res {
    IResult::Done(i, o) => {
      if i.len() == 0 {
        Result::Ok(o)
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