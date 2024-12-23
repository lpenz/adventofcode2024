// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day21::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|seq| {
            let fullseqlen = numpad_sequence_len(25, &seq);
            let numericpart = numericpart_calc(&seq);
            fullseqlen * numericpart
        })
        .sum())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
