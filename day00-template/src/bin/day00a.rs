// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day00::*;

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    Ok(input.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 1);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
