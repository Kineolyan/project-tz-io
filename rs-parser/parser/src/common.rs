use nom::IResult;
use nom::Parser;
use nom::character::complete::digit1;
use nom::character::complete::space0;
use nom::combinator as c;
use nom_language::error::VerboseError;

use std::str;

/// Result of all parsers of this crate, carrying verbose errors
pub type Res<'a, O> = IResult<&'a [u8], O, VerboseError<&'a [u8]>>;

/// Builds a parsing error at the given position
pub fn error_at(input: &[u8], kind: nom::error::ErrorKind) -> VerboseError<&[u8]> {
    nom::error::ParseError::from_error_kind(input, kind)
}

/// Wrap a parser with space-consumers
pub fn ws<'a, F, O, E: nom::error::ParseError<&'a [u8]>>(
    inner: F,
) -> impl Parser<&'a [u8], Output = O, Error = E>
where
    F: Parser<&'a [u8], Output = O, Error = E>,
{
    nom::sequence::delimited(space0, inner, space0)
}

#[cfg(test)]
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

pub fn be_uint(input: &[u8]) -> Res<'_, u32> {
    c::map_res(digit1, to::<u32>).parse(input)
}

pub fn be_u8(input: &[u8]) -> Res<'_, u8> {
    let (input, number) = c::map_res(digit1, to::<u8>).parse(input)?;
    Ok((input, number))
}

pub fn be_i8(input: &[u8]) -> Res<'_, i8> {
    let (input, sign) = nom::combinator::opt(nom::bytes::complete::tag("-")).parse(input)?;
    let (input, number) = c::map_res(digit1, to::<i8>).parse(input)?;
    let value = match sign {
        Some(_) => -number,
        None => number,
    };
    Ok((input, value))
}

/// Consume a one-line comment without consuming the new-line chars
fn end_line_comment(input: &[u8]) -> Res<'_, ()> {
    nom::combinator::value(
        (),
        nom::sequence::pair(
            nom::bytes::complete::tag("//"),
            nom::bytes::complete::is_not("\n\r"),
        ),
    ).parse(input)
}

/// Parses all spaces until the new-line, including an optional single-line comment
pub fn eol(input: &[u8]) -> Res<'_, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = nom::combinator::opt(end_line_comment).parse(input)?;
    nom::combinator::value((), nom::character::complete::newline).parse(input)
}

pub fn opt_eol(input: &[u8]) -> Res<'_, Vec<()>> {
    nom::multi::many0(eol).parse(input)
}

#[cfg(test)]
pub mod tests {
    use std::cmp::PartialEq;
    use std::fmt::Debug;

    use super::*;
    use nom::Err;

    fn assert_remaining_content(value: &[u8], expected: &[u8]) {
        if value != expected {
            panic!(
                "Unexpected remaining ```{}``` != ```{}```",
                str::from_utf8(value).unwrap(),
                str::from_utf8(expected).unwrap()
            );
        }
    }
    pub fn assert_result<Result: PartialEq + Debug>(
        res: Res<'_, Result>,
        value: Result,
        remaining: &[u8],
    ) {
        match res {
            Ok((parsed, result)) => {
                assert_eq!(result, value);
                assert_remaining_content(parsed, remaining);
            }
            Err(nom::Err::Failure(error)) => {
                let input = error.errors.first().map_or(&b""[..], |(i, _)| i);
                panic!("Cannot parse ```{}```", str::from_utf8(input).unwrap());
            }
            _ => assert_eq!(res, Ok((remaining, value))),
        }
    }

    pub fn assert_full_result<Result: PartialEq + Debug>(
        res: Res<'_, Result>,
        value: Result,
    ) {
        if let Ok((remaining, _)) = &res {
            if remaining.len() > 0 as usize {
                panic!(
                    "Unexpected remaining {}",
                    str::from_utf8(remaining).unwrap()
                );
            }
        }
        assert_result(res, value, to_input(b""));
    }

    pub fn assert_cannot_parse<Result: PartialEq + Debug>(res: Res<'_, Result>) {
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
