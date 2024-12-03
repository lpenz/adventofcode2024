// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

use regex::Regex;

fn process(bufin: impl BufRead) -> Result<u64> {
    let input = std::io::read_to_string(bufin)?;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    Ok(re
        .captures_iter(&input)
        .map(|m| {
            let (_, [n1, n2]) = m.extract();
            n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 161);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
