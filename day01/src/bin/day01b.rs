// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

use std::collections::HashMap;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let counts = right
        .into_iter()
        .fold(HashMap::<i32, i32>::default(), |mut counts, i| {
            *counts.entry(i).or_default() += 1;
            counts
        });
    let score = left
        .into_iter()
        .map(|i| i * counts.get(&i).unwrap_or(&0))
        .sum();
    Ok(score)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 31);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
