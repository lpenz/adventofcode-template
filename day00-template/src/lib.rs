// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "0\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> PResult<&str, u8> {
        nom::error::context("cannot parse u8", map_res(character::u32, u8::try_from)).parse(input)
    }

    fn line(input: &str) -> PResult<&str, u8> {
        let (input, num) = context("num err", num).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(input: &str) -> Result<Vec<u8>> {
        aoc::parse_with!(multi::many1(line), input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input.len(), 1);
    Ok(())
}
