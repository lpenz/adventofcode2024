// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day22::*;

fn process(bufin: impl BufRead) -> Result<Num> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|mut s| {
            for _ in 0..2000 {
                s = evolve(s);
            }
            s
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 37327623);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
