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
pub mod parser;

/*****************************************************************************/

/// Extend Option with ok_or_eyre
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

/*****************************************************************************/

/// Wrapper that adds Eq and Ord using Debug
#[derive(Debug)]
pub struct OrdWrapper<T>(pub T);

impl<T: Debug> std::cmp::PartialEq for OrdWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self).eq(&format!("{:?}", other))
    }
}

impl<T: Debug> Eq for OrdWrapper<T> {}

impl<T: Debug> std::cmp::PartialOrd for OrdWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Debug> std::cmp::Ord for OrdWrapper<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

/*****************************************************************************/

// main function

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
