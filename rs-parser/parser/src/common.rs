use nom::character::complete::digit1;
use nom::combinator as c;
use nom::IResult;

use std::str;

pub fn to_input(v: &[u8]) -> &[u8] {
	v
}

pub fn to_string(v: &[u8]) -> Result<String, i8> {
	str::from_utf8(v).map(|s| s.to_string()).or(Err(-1))
}

fn to<T: str::FromStr>(v: &[u8]) -> Result<T, i8> {
	str::from_utf8(v)
		.or(Err(-1))
		.and_then(|i| i.parse::<T>().or(Err(-2)))
}

pub fn be_uint(input: &[u8]) -> IResult<&[u8], u32> {
	c::map_res(digit1, to::<u32>)(input)
}

pub fn be_u8(input: &[u8]) -> IResult<&[u8], u8> {
	c::map_res(digit1, to::<u8>)(input)
}

pub fn be_i8(input: &[u8]) -> IResult<&[u8], i8> {
	let (input, sign) = nom::combinator::opt(nom::bytes::complete::tag("-"))(input)?;
	let (input, number) = c::map_res(digit1, to::<i8>)(input)?;
	let value = match sign {
		Some(_) => -number,
		None => number,
	};
	Ok((input, value))
}

// pub fn be_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
// where
//   I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
// {
//   let bound: usize = 4;
//   if input.input_len() < bound {
//     Err(Err::Error(make_error(input, ErrorKind::Eof)))
//   } else {
//     let mut res = 0u32;
//     for byte in input.iter_elements().take(bound) {
//       res = (res << 8) + byte as u32;
//     }

//     Ok((input.slice(bound..), res))
//   }
// }

fn end_line_comment(input: &[u8]) -> IResult<&[u8], ()> {
	// alt!(
	// 	do_parse!(tag!("//\n") >> ()) |
	// 	do_parse!(tag!("//") >> is_not!("/\n") >> take_until!("\n") >> ())
	// )
	todo!()
}
pub fn eol(input: &[u8]) -> IResult<&[u8], ()> {
	// do_parse!(
	// 	ospace >>
	// 	opt!(end_line_comment) >>
	// 	tag!("\n") >>
	// 	()
	// )
	todo!()
}

pub fn opt_eol(input: &[u8]) -> IResult<&[u8], Vec<()>> {
	nom::multi::many0(eol)(input)
}

#[cfg(test)]
pub mod tests {
	use std::cmp::PartialEq;
	use std::fmt::Debug;

	use super::*;
	use nom::{Err, IResult};

	pub fn assert_result<Result: PartialEq + Debug>(
		res: IResult<&[u8], Result>,
		value: Result,
		remaining: &[u8],
	) {
		assert_eq!(res, Ok((remaining, value)));
	}

	pub fn assert_full_result<Result: PartialEq + Debug>(res: IResult<&[u8], Result>, value: Result) {
		if let &Ok((ref remaining, _)) = &res {
			if remaining.len() > 0 {
				println!(
					"Unexpected remaining {}",
					str::from_utf8(remaining).unwrap()
				);
			}
		}
		assert_result(res, value, to_input(b""));
	}

	pub fn assert_cannot_parse<Result: PartialEq + Debug>(res: IResult<&[u8], Result>) {
		match res {
			Ok((i, o)) => {
				panic!(
					"Unexpected successful parsing. Res {:?}, remaining {:?}",
					o, i
				);
			}
			Err(Err::Incomplete(needed)) => {
				panic!("Cannot parse due to missing data. Needed {:?}", needed);
			}
			Err(Err::Error(_)) | Err(Err::Failure(_)) => {
				// Ok, nothing to do
			}
		}
	}

	pub fn assert_incomplete<Result: PartialEq + Debug>(res: IResult<&[u8], Result>) {
		match res {
			Ok((i, o)) => {
				panic!(
					"Unexpected successful parsing. Res {:?}, remaining {:?}",
					o, i
				);
			}
			Err(Err::Incomplete(_)) => {
				// Ok, nothing to do
			}
			Err(Err::Error(e)) | Err(Err::Failure(e)) => {
				panic!("Unexpected error while parsing: {:?}", e);
			}
		}
	}

	#[test]
	fn test_parse_be_u8() {
		let res = be_u8(b"42");
		assert_full_result(res, 42);
	}

	#[test]
	fn test_parse_be_u8_zero() {
		let res = be_u8(b"0");
		assert_full_result(res, 0);
	}

	#[test]
	fn test_parse_be_i8_positive() {
		let res = be_i8(b"13");
		assert_full_result(res, 13);
	}

	#[test]
	fn test_parse_be_i8_negative() {
		let res = be_i8(b"-7");
		assert_full_result(res, -7);
	}

	#[test]
	fn test_parse_be_i8_zero() {
		let res = be_i8(b"0");
		assert_full_result(res, 0);
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
		let res = opt_eol(to_input(
			b"

	// Some comment

// and multi
// lines with comments
next",
		));
		let (remaining, _) = res.unwrap();
		assert_eq!(remaining, to_input(b"next"));
	}
}
