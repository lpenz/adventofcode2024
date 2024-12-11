// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day07::*;
use rayon::prelude::*;

fn fix(_eq: &Equation, target: N, curr: N, operands: &[N]) -> bool {
    if curr > target {
        return false;
    }
    if operands.is_empty() {
        return curr == target;
    }
    if fix(_eq, target, curr + operands[0], &operands[1..]) {
        return true;
    }
    if fix(_eq, target, curr * operands[0], &operands[1..]) {
        return true;
    }
    let s = format!("{}", operands[0]);
    let mul = std::iter::repeat(10).take(s.len()).product::<N>();
    fix(_eq, target, curr * mul + operands[0], &operands[1..])
}

fn process(bufin: impl BufRead) -> Result<N> {
    let equations = parser::parse(bufin)?;
    Ok(equations
        .into_par_iter()
        .filter_map(|eq| fix(&eq, eq.0, 0, &eq.1).then_some(eq.0))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 11387);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
