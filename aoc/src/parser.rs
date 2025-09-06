// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::Report;
pub use color_eyre::Result;
pub use color_eyre::eyre::eyre;
pub use combinator::all_consuming;
pub use nom::Finish;
pub use nom::IResult;
pub use nom::Parser;
pub use nom::branch;
pub use nom::bytes::complete as bytes;
pub use nom::bytes::complete::tag;
pub use nom::character::complete as character;
pub use nom::character::complete::newline;
pub use nom::character::complete::satisfy;
pub use nom::combinator;
pub use nom::combinator::map_res;
pub use nom::error::context;
pub use nom::multi;
use nom_language::error::VerboseError;
pub use std::io::BufRead;

pub type PResult<I, O, E = VerboseError<I>> = Result<(I, O), nom::Err<E>>;

#[macro_export]
macro_rules! parse_with {
    ($parser:expr, $input:ident) => {{
        let result = all_consuming($parser).parse(&$input).finish();
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }};
}

pub fn space(input: &str) -> IResult<&str, &str> {
    tag(" ")(input)
}

pub fn digit1_one_of<'a, E>(
    valid: &str,
) -> impl FnMut(&'a str) -> IResult<&'a str, u8, E> + use<'a, '_, E>
where
    E: nom::error::ParseError<&'a str>,
{
    move |input| {
        let (input, c) = character::one_of(valid)(input)?;
        Ok((input, c.to_digit(10).unwrap() as u8))
    }
}

pub fn digit1(input: &str) -> IResult<&str, u8> {
    digit1_one_of("0123456789")(input)
}

pub fn lowercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_lowercase())(input)
}

pub fn lowercase_str(input: &str) -> IResult<&str, String> {
    let (input, cs) = multi::many1(lowercase_char).parse(input)?;
    Ok((input, cs.into_iter().collect()))
}

pub fn grid_line<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    E: nom::error::ParseError<&'a str>,
    F: nom::Parser<&'a str, Output = O, Error = E>,
    F: Copy,
{
    move |input| {
        let (input, cell) = multi::many1(f).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cell))
    }
}

pub fn grid<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Vec<O>>, E>
where
    E: nom::error::ParseError<&'a str>,
    F: nom::Parser<&'a str, Output = O, Error = E>,
    F: Copy,
{
    move |input| multi::many1(grid_line(f)).parse(input)
}
