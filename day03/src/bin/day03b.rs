// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

fn process(bufin: impl BufRead) -> Result<u64> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .fold((true, 0_u64), |(enabled, sum), instr| match instr {
            Instr::Do => (true, sum),
            Instr::Dont => (false, sum),
            Instr::Mul(n1, n2) => (enabled, if enabled { sum + n1 * n2 } else { sum }),
        })
        .1)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 48);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
