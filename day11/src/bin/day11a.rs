// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

fn process(num: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(solve(num, &input))
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(6, "125 17\n".as_bytes())?, 22);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(25, "125 17\n".as_bytes())?, 55312);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(25, stdin().lock()))
}
