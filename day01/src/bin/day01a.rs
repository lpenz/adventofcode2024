// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    left.sort();
    right.sort();
    let result = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    Ok(result)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 11);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
