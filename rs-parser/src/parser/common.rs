use nom::{digit, space, newline};

use std::str;

pub type RawData = [u8];

pub fn to_string(v: &RawData) -> Result<String, i8> {
	str::from_utf8(v).map(|s| s.to_string()).or(Err(-1))
}

fn to<T: str::FromStr>(v: &RawData) -> Result<T, i8> {
	str::from_utf8(v).or(Err(-1))
		.and_then(|i| i.parse::<T>().or(Err(-2)))

}

fn to_u8(v: &RawData) -> Result<u8, i8> {
	to(v)
}

fn to_i8(v: &RawData) -> Result<i8, i8> {
	to(v)
}

fn to_u32(v: &RawData) -> Result<u32, i8> {
	to(v)
}

named!(pub be_uint<&RawData, u32>, map_res!(digit, to_u32));
named!(pub be_u8<&RawData, u8>, map_res!(digit, to_u8));
named!(pub be_i8<&RawData, i8>, 
	do_parse!(
		s: opt!(tag!("-")) >>
		d: digit >>
		(to_i8(d).map(|value|
			if s.is_some() { -value } else { value }
		).expect("Not a number"))
	)
);
named!(pub ospace<&RawData, Option<&RawData> >, opt!(space));
named!(end_line_comment<&RawData, ()>,
	alt!(
		do_parse!(tag!("//\n") >> ()) |
		do_parse!(tag!("//") >> is_not!("/\n") >> take_until!("\n") >> ())
	)
);
named!(pub eol<&RawData, ()>,
	do_parse!(
		ospace >>
		opt!(end_line_comment) >>
		newline >>
		()
	)
);
named!(pub opt_eol<&RawData, Vec<()> >, many0!(eol));

#[cfg(test)]
pub mod tests {
	use std::cmp::PartialEq;
	use std::fmt::Debug;

	use super::*;
	use nom::{Err, IResult};

	pub fn assert_result<Result: PartialEq + Debug> (
			res: IResult<&[u8], Result>,
			value: Result,
			remaining: &RawData) {
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
		assert_result(res, value, b"");
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
	fn test_parse_be_uint() {
		let input = b"14";
		let res = be_uint(input);
		assert_full_result(res, 14u32);
	}

	#[test]
	fn test_parse_be_u8() {
		let input = b"4";
		let res = be_u8(input);
		assert_full_result(res, 4u8);
	}

	#[test]
	fn test_parse_be_i8_positive() {
		let input = b"123";
		let res = be_i8(input);
		assert_full_result(res, 123i8);
	}

	#[test]
	fn test_parse_be_i8_negative() {
		let input = b"-98";
		let res = be_i8(input);
		assert_full_result(res, -98i8);
	}

	#[test]
	fn test_parse_end_line_comment() {
		let res = end_line_comment(b"// some comment\nnext");
		assert_result(res, (), b"\nnext");
	}

	#[test]
	fn test_parse_eol_with_comment() {
		let res = eol(b"// eol with comment\nnext");
		assert_result(res, (), b"next");
	}

	#[test]
	fn test_parse_eol_with_indented_comment() {
		let res = eol(b"  	// eol with comment\nnext");
		assert_result(res, (), b"next");
	}

	#[test]
	fn test_parse_multiline_combining_comment_and_spaces() {
		let res = opt_eol(b"

	// Some comment

// and multi
// lines with comments
next");
		let (remaining, _) = res.unwrap();
		assert_eq!(remaining, b"next");
	}

}