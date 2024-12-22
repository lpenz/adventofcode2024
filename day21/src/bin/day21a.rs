// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day21::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|seq| {
            let fullseq = full_sequence(2, &seq);
            let numericpart = numericpart_calc(&seq);
            fullseq.len() * numericpart
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 126384);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
