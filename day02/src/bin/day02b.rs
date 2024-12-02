// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

fn is_safe(report: &[i32]) -> bool {
    let increasing = report[1] - report[0] > 0;
    std::iter::zip(report.iter(), report.iter().skip(1))
        .all(|(i, j)| increasing && i < j && j - i <= 3 || !increasing && i > j && i - j <= 3)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let num_safe = input
        .iter()
        .filter(|report| {
            if is_safe(report) {
                true
            } else {
                (0..report.len()).any(|i| {
                    let mut report2 = report.to_vec();
                    report2.remove(i);
                    is_safe(&report2)
                })
            }
        })
        .count();
    Ok(num_safe)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 4);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
