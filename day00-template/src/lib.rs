// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "0\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, u32> {
        character::u32(input)
    }

    fn line(input: &str) -> IResult<&str, u32> {
        let (input, num) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<u32>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 1);
    Ok(())
}
