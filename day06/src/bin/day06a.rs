// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;

use sqrid::Dir;

use day06::*;

fn process(bufin: impl BufRead, size: u16) -> Result<usize> {
    let (walls, mut guard) = parser::parse(bufin)?;
    let mut visited = HashSet::<Pos>::default();
    visited.insert(guard);
    let mut d = Dir::N;
    loop {
        let Ok(next) = guard + d else { break };
        if next.x() >= size || next.y() >= size {
            break;
        }
        if walls.contains(&next) {
            d += Dir::E;
        } else {
            guard = next;
            visited.insert(guard);
        }
    }
    Ok(visited.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes(), 10)?, 41);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock(), 130))
}
