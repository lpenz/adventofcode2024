// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let safe = input
        .iter()
        .filter(|report| {
            let increasing = report[1] - report[0] > 0;
            std::iter::zip(report.iter(), report.iter().skip(1)).all(|(i, j)| {
                increasing && i < j && j - i <= 3 || !increasing && i > j && i - j <= 3
            })
        })
        .count();
    Ok(safe)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
