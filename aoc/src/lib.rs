// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt::{Debug, Display};
pub use std::io::{stdin, BufRead};
use std::time::Instant;

pub use color_eyre::eyre::eyre;
pub use color_eyre::Report;
pub use color_eyre::Result;

#[macro_use]
pub mod parser {
    pub use color_eyre::eyre::eyre;
    pub use color_eyre::Report;
    pub use color_eyre::Result;
    pub use combinator::all_consuming;
    pub use nom::branch;
    pub use nom::bytes::complete as bytes;
    pub use nom::bytes::complete::tag;
    pub use nom::character::complete as character;
    pub use nom::character::complete::newline;
    pub use nom::character::complete::satisfy;
    pub use nom::combinator;
    pub use nom::multi;
    pub use nom::Finish;
    pub use nom::IResult;
    pub use std::io::BufRead;

    #[macro_export]
    macro_rules! parse_with {
        ($parser:expr, $buf:ident) => {{
            let mut input = String::default();
            $buf.read_to_string(&mut input)?;
            let result = all_consuming($parser)(&input).finish();
            Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
        }};
    }

    pub fn space(input: &str) -> IResult<&str, &str> {
        tag(" ")(input)
    }

    pub fn lowercase_char(input: &str) -> IResult<&str, char> {
        satisfy(|c| c.is_ascii_lowercase())(input)
    }

    pub fn lowercase_str(input: &str) -> IResult<&str, String> {
        let (input, cs) = multi::many1(lowercase_char)(input)?;
        Ok((input, cs.into_iter().collect()))
    }
}

pub trait OptionExt<T> {
    fn ok_or_eyre<M>(self, message: M) -> Result<T>
    where
        M: Debug + Display + Send + Sync + 'static;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_eyre<M>(self, message: M) -> Result<T>
    where
        M: Debug + Display + Send + Sync + 'static,
    {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Report::msg(message)),
        }
    }
}

pub fn elapsed(start: &Instant) -> String {
    format!("{}", humantime::Duration::from(start.elapsed()))
}

pub fn do_main<F: Fn() -> Result<T>, T: Display>(f: F) -> Result<()> {
    color_eyre::install()?;
    let start = Instant::now();
    println!("{}", f()?);
    println!("Elapsed: {}", elapsed(&start));
    Ok(())
}
