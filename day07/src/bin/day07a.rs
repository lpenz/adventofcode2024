// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day07::*;

fn fix(target: N, curr0: N, operands: &[N]) -> bool {
    if curr0 > target {
        return false;
    }
    if operands.is_empty() {
        return curr0 == target;
    }
    if fix(target, curr0 + operands[0], &operands[1..]) {
        return true;
    }
    fix(target, curr0 * operands[0], &operands[1..])
}

fn process(bufin: impl BufRead) -> Result<N> {
    let equations = parser::parse(bufin)?;
    Ok(equations
        .into_iter()
        .filter_map(|eq| fix(eq.0, 0, &eq.1).then_some(eq.0))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 3749);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
