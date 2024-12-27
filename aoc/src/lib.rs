// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt::{Debug, Display};
pub use std::io::stdin;
pub use std::io::BufRead;
pub use std::io::BufReader;
pub use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;

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

#[derive(Parser)]
#[command()]
struct Cli {
    /// Input file
    filename: PathBuf,

    /// Quiet mode: don't print reponse nor elapsed time
    #[arg(short, long)]
    quiet: bool,
}

pub fn elapsed(start: &Instant) -> String {
    format!("{}", humantime::Duration::from(start.elapsed()))
}

pub fn do_main<F: Fn(&str) -> Result<T>, T: Display>(f: F) -> Result<()> {
    color_eyre::install()?;
    let start = Instant::now();
    let cli = Cli::parse();
    let contents = std::fs::read_to_string(cli.filename)?;
    let result = f(&contents)?;
    if !cli.quiet {
        println!("{}", result);
        println!("Elapsed: {}", elapsed(&start));
    }
    Ok(())
}
