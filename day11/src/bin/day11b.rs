// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

fn process(num: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(solve(num, &input))
}

fn main() -> Result<()> {
    do_main(|| process(75, stdin().lock()))
}
