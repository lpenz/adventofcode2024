// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day20::*;

use rayon::prelude::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use sqrid::postrait::PosT;

pub type Cost = usize;

pub fn find_path(g: &Grid, start: Pos) -> Cost {
    let mut frontier = BinaryHeap::<(Reverse<Cost>, Pos)>::new();
    frontier.push((Reverse(0), start));
    let mut costmap = HashMap::new();
    costmap.insert(start, 0);
    while let Some((_, pos0)) = frontier.pop() {
        if g[pos0] == Cell::End {
            return costmap[&pos0];
        }
        let newcost = costmap[&pos0] + 1;
        for dir in Dir::iter::<false>() {
            let Some(pos) = (pos0 + dir).ok().filter(|p| g[p] != Cell::Wall) else {
                continue;
            };
            if g[&pos] == Cell::Wall {
                continue;
            }
            let e = costmap.entry(pos).or_insert(usize::MAX);
            if newcost < *e {
                *e = newcost;
                frontier.push((Reverse(newcost), pos));
            }
        }
    }
    panic!("path not found")
}

fn check_neighs_empty(g: &Grid, p0: Pos, d1: Dir, d2: Dir) -> bool {
    let Ok(p1) = p0 + d1 else {
        return false;
    };
    let Ok(p2) = p0 + d2 else {
        return false;
    };
    g[p1] != Cell::Wall && g[p2] != Cell::Wall
}

fn do_cheat(g: &Grid, cheat: Pos, start: Pos) -> Option<Cost> {
    if g[cheat] != Cell::Wall {
        return None;
    }
    if !check_neighs_empty(g, cheat, Dir::N, Dir::S)
        && !check_neighs_empty(g, cheat, Dir::W, Dir::E)
    {
        return None;
    }
    let mut gcheat = *g;
    gcheat[cheat] = Cell::Empty;
    Some(find_path(&gcheat, start))
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let g = parser::parse(bufin)?;
    let start = grid_find(&g, Cell::Start);
    let cost_base = find_path(&g, start);
    Ok(Pos::iter()
        .par_bridge()
        .filter(|p| {
            let Some(newcost) = do_cheat(&g, *p, start) else {
                return false;
            };
            cost_base - newcost >= 100
        })
        .count())
}

#[test]
fn test_find_path() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let start = grid_find(&g, Cell::Start);
    assert_eq!(find_path(&g, start), 84);
    Ok(())
}

#[test]
fn test_cheat1() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let start = grid_find(&g, Cell::Start);
    assert_eq!(do_cheat(&g, Pos::new_static::<8, 1>(), start), Some(72));
    Ok(())
}

#[test]
fn test_cheat2() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let start = grid_find(&g, Cell::Start);
    assert_eq!(do_cheat(&g, Pos::new_static::<10, 7>(), start), Some(64));
    Ok(())
}

#[test]
fn test_cheat3() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let start = grid_find(&g, Cell::Start);
    assert_eq!(
        do_cheat(&g, Pos::new_static::<8, 8>(), start),
        Some(84 - 38)
    );
    Ok(())
}

#[test]
fn test_cheat4() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let start = grid_find(&g, Cell::Start);
    assert_eq!(
        do_cheat(&g, Pos::new_static::<6, 7>(), start),
        Some(84 - 64)
    );
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
