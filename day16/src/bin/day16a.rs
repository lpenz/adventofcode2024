// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day16::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let g = parser::parse(bufin)?;
    calc_best(&g)
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
