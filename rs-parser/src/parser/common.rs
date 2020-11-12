use nom::IResult;
use nom::{digit, space};
use nom::types::CompleteByteSlice;

use std::str;

pub fn to_input(content: &[u8]) -> &[u8] {
	CompleteByteSlice(content)
}
pub fn from_input<'a>(content: Input<'a>) -> &'a[u8] {
	content.0
}

pub fn to_string(v: Input) -> Result<String, i8> {
	str::from_utf8(v.0).map(|s| s.to_string()).or(Err(-1))
}

fn end_line_comment(input: Input) -> IResult<&[u8], ()> {
	alt!(
		do_parse!(tag!("//\n") >> ()) |
		do_parse!(tag!("//") >> is_not!("/\n") >> take_until!("\n") >> ())
	)
);
pub fn eol(input: Input) -> IResult<&[u8], ()> {
	do_parse!(
		nom::character::complete::space0 >>
		opt!(end_line_comment) >>
		tag!("\n") >>
		()
	)
);
pub fn opt_eol(input: Input) -> IResult<&[u8], Vec<()> > { many0!(eol));

#[cfg(test)]
pub mod tests {
	use std::cmp::PartialEq;
	use std::fmt::Debug;

	use super::*;
	use nom::{Err, IResult};

	pub fn assert_result<Result: PartialEq + Debug> (
			res: IResult<&[u8], Result>,
			value: Result,
			remaining: Input) {
		assert_eq!(
			res,
			Ok((remaining, value))
		);
	}

	pub fn assert_full_result<Result: PartialEq + Debug> (
			res: IResult<&[u8], Result>,
			value: Result) {
		if let &Ok((ref remaining, _)) = &res {
			if remaining.len() > 0 {
				println!("Unexpected remaining {}", str::from_utf8(remaining).unwrap());
			}
		}
		assert_result(res, value, to_input(b""));
	}

	pub fn assert_cannot_parse<Result: PartialEq + Debug>(res: IResult<&[u8], Result>) {
		match res {
			Ok((i, o)) => {
				panic!("Unexpected successful parsing. Res {:?}, remaining {:?}", o, i);
			},
			Err(Err::Incomplete(needed)) => {
				panic!("Cannot parse due to missing data. Needed {:?}", needed);
			},
			Err(Err::Error(_)) | Err(Err::Failure(_)) => {
				// Ok, nothing to do
			}
		}
	}

	pub fn assert_incomplete<Result: PartialEq + Debug>(res: IResult<&[u8], Result>) {
		match res {
			Ok((i, o)) => {
				panic!("Unexpected successful parsing. Res {:?}, remaining {:?}", o, i);
			},
			Err(Err::Incomplete(_)) => {
				// Ok, nothing to do
			},
			Err(Err::Error(e)) | Err(Err::Failure(e)) => {
				panic!("Unexpected error while parsing: {:?}", e);
			}
		}
	}

	#[test]
	fn test_parse_end_line_comment() {
		let res = end_line_comment(to_input(b"// some comment\nnext"));
		assert_result(res, (), to_input(b"\nnext"));
	}

	#[test]
	fn test_parse_eol_with_comment() {
		let res = eol(to_input(b"// eol with comment\nnext"));
		assert_result(res, (), to_input(b"next"));
	}

	#[test]
	fn test_parse_eol_with_indented_comment() {
		let res = eol(to_input(b"  	// eol with comment\nnext"));
		assert_result(res, (), to_input(b"next"));
	}

	#[test]
	fn test_parse_multiline_combining_comment_and_spaces() {
		let res = opt_eol(to_input(b"

	// Some comment

// and multi
// lines with comments
next"));
		let (remaining, _) = res.unwrap();
		assert_eq!(remaining, to_input(b"next"));
	}

}