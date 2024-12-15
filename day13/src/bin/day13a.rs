// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day13::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn calc(a: XY, b: XY, prize: XY) -> usize {
    let mut frontier = BinaryHeap::<(Reverse<usize>, usize, usize, OrdWrapper<XY>)>::new();
    frontier.push((Reverse(0), 0, 0, OrdWrapper(XY::new(0, 0))));
    let mut visited = HashSet::new();
    while let Some((cost, atimes, btimes, OrdWrapper(pos))) = frontier.pop() {
        if pos == prize {
            return cost.0;
        }
        let key = (cost.0, pos);
        if pos.re > prize.re || pos.im > prize.im || visited.contains(&key) {
            continue;
        }
        visited.insert(key);
        if atimes < 100 {
            frontier.push((Reverse(cost.0 + 3), atimes + 1, btimes, OrdWrapper(pos + a)));
        }
        if btimes < 100 {
            frontier.push((Reverse(cost.0 + 1), atimes, btimes + 1, OrdWrapper(pos + b)));
        }
    }
    0
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let machines = parser::parse(bufin)?;
    Ok(machines
        .into_iter()
        .map(|(a, b, prize)| calc(a, b, prize))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 480);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
