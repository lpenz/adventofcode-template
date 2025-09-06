// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt::{Debug, Display};
pub use std::io::BufRead;
pub use std::io::BufReader;
pub use std::io::Read;
pub use std::io::stdin;
use std::path::PathBuf;
use std::time::Instant;

use argh::FromArgs;

pub use color_eyre::Report;
pub use color_eyre::Result;
pub use color_eyre::eyre::OptionExt;
pub use color_eyre::eyre::WrapErr;
pub use color_eyre::eyre::eyre;

#[macro_use]
pub mod parser;

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

#[derive(FromArgs)]
#[argh(description = "Run challenge code against the input file")]
#[argh(help_triggers("-h", "--help"))]
struct Cli {
    /// input file
    #[argh(positional)]
    filename: PathBuf,

    /// quiet mode - don't print reponse nor elapsed time
    #[argh(switch, short = 'q')]
    quiet: bool,
}

pub fn elapsed(start: &Instant) -> String {
    format!("{}", humantime::Duration::from(start.elapsed()))
}

pub fn do_main<F: Fn(&str) -> Result<T>, T: Display>(f: F) -> Result<()> {
    color_eyre::install()?;
    let start = Instant::now();
    let cli: Cli = argh::from_env();
    let contents = std::fs::read_to_string(cli.filename.clone())
        .wrap_err_with(|| format!("error reading file {}", cli.filename.display()))?;
    let result = f(&contents).wrap_err("error evaluating challenge solution on input")?;
    if !cli.quiet {
        println!("{}", result);
        println!("Elapsed: {}", elapsed(&start));
    }
    Ok(())
}
