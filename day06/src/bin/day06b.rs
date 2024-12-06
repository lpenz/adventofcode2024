// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use std::collections::HashMap;
use std::collections::HashSet;

use sqrid::Dir;

use day06::*;

fn get_next(size: u16, guard: Pos, d: Dir) -> Option<Pos> {
    (guard + d).ok().filter(|n| n.x() < size && n.y() < size)
}

fn check_cycle(mut guard: Pos, walls: &HashSet<Pos>, size: u16, new_wall: Pos) -> bool {
    let mut d = Dir::N;
    let mut visited = HashSet::<(Pos, Dir)>::default();
    loop {
        let Some(next) = get_next(size, guard, d) else {
            return false;
        };
        if walls.contains(&next) || next == new_wall {
            d += Dir::E;
        } else {
            if visited.contains(&(guard, d)) {
                return true;
            }
            visited.insert((guard, d));
            guard = next;
        }
    }
}

fn do_process(bufin: impl BufRead, size: u16) -> Result<HashSet<Pos>> {
    let (walls, guard0) = parser::parse(bufin)?;
    let mut guard = guard0;
    // Extra obstructions that would create a cycle
    let mut extra = HashSet::<Pos>::default();
    let mut d = Dir::N;
    loop {
        let Some(next) = get_next(size, guard, d) else {
            break;
        };
        if walls.contains(&next) {
            d += Dir::E;
        } else {
            if next != guard0 {
                // Check what happens if we put a rock right at next:
                if check_cycle(guard0, &walls, size, next) {
                    extra.insert(next);
                }
            }
            guard = next;
        }
    }
    Ok(extra)
}

fn process(bufin: impl BufRead, size: u16) -> Result<usize> {
    Ok(do_process(bufin, size)?.len())
}

#[test]
fn test_found() -> Result<()> {
    let mut ans = vec![
        Pos::new_unwrap(3, 6),
        Pos::new_unwrap(6, 7),
        Pos::new_unwrap(7, 7),
        Pos::new_unwrap(1, 8),
        Pos::new_unwrap(3, 8),
        Pos::new_unwrap(7, 9),
    ];
    ans.sort();
    let mut calc = do_process(EXAMPLE.as_bytes(), 10)?
        .into_iter()
        .collect::<Vec<_>>();
    calc.sort();
    assert_eq!(calc, ans);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes(), 10)?, 6);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock(), 130))
}
