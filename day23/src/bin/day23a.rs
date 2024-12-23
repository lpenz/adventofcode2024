// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day23::*;

use std::collections::HashSet;

use itertools::Itertools;

fn process(bufin: impl BufRead) -> Result<usize> {
    let connections = parser::parse(bufin)?;
    let cpus = connections
        .iter()
        .flat_map(|conn| [conn.0, conn.1].into_iter())
        .collect::<HashSet<_>>();
    let connections = connections.into_iter().collect::<HashSet<_>>();
    let mut count = 0;
    for cpucomb in cpus.iter().combinations(3) {
        let [cpu1, cpu2, cpu3] = cpucomb[..] else {
            panic!()
        };
        if ![cpu1, cpu2, cpu3].into_iter().any(|c| c.prefix_t()) {
            continue;
        }
        if !connections.contains(&connect(*cpu1, *cpu2))
            || !connections.contains(&connect(*cpu2, *cpu3))
            || !connections.contains(&connect(*cpu1, *cpu3))
        {
            continue;
        }
        count += 1;
    }
    Ok(count)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 7);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
