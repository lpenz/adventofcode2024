// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day20::*;

// use rayon::prelude::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use sqrid::postrait::PosT;

pub type Cost = usize;
pub type CostMap = sqrid::grid_create!(Sqrid, Cost);

pub fn costmap_calc(g: &Grid, end: Pos) -> CostMap {
    let mut frontier = BinaryHeap::<(Reverse<Cost>, Pos)>::new();
    frontier.push((Reverse(0), end));
    let mut costmap = CostMap::repeat(Cost::MAX);
    costmap[end] = 0;
    while let Some((_, pos0)) = frontier.pop() {
        let newcost = costmap[pos0] + 1;
        for dir in Dir::iter::<false>() {
            let Some(pos) = (pos0 + dir).ok().filter(|p| g[p] != Cell::Wall) else {
                continue;
            };
            let oldcost = costmap[pos];
            if newcost < oldcost {
                costmap[pos] = newcost;
                frontier.push((Reverse(newcost), pos));
            }
        }
    }
    costmap
}

type CheatsMap = HashMap<(Pos, Pos), Cost>;

pub fn cheats_pos_calc(
    _g: &Grid,
    cost_to_end_map: &CostMap,
    cost_so_far: Cost,
    cheatstart: Pos,
    cheatsmap: &mut CheatsMap,
) {
    let y0 = cheatstart.y().saturating_sub(20);
    let y1 = cheatstart.y().saturating_add(20);
    let x0 = cheatstart.x().saturating_sub(20);
    let x1 = cheatstart.x().saturating_add(20);
    for y in y0..=y1 {
        for x in x0..=x1 {
            let Ok(pos) = Pos::new(x, y) else {
                continue;
            };
            if cost_to_end_map[pos] == Cost::MAX {
                continue;
            }
            let dist = Pos::manhattan(&cheatstart, &pos);
            if dist > 20 {
                continue;
            }
            let newcost = cost_so_far + dist as Cost + cost_to_end_map[pos];
            let e = cheatsmap.entry((cheatstart, pos)).or_insert(Cost::MAX);
            if newcost < *e {
                *e = newcost;
            }
        }
    }
}

pub fn cheats_all_calc(g: &Grid) -> HashMap<Cost, usize> {
    let start = grid_find(g, Cell::Start);
    let cost_to_start_map = costmap_calc(g, start);
    let end = grid_find(g, Cell::End);
    let cost_to_end_map = costmap_calc(g, end);
    let mut cheatsmap = CheatsMap::default();
    for (pos, poscost) in cost_to_start_map.iter_pos() {
        if *poscost == Cost::MAX {
            continue;
        }
        cheats_pos_calc(g, &cost_to_end_map, *poscost, pos, &mut cheatsmap);
    }
    let base = cost_to_end_map[start];
    cheatsmap
        .into_iter()
        .fold(Default::default(), |mut costfreq, ((_, _), cost)| {
            if base > cost {
                let save = base - cost;
                let e = costfreq.entry(save).or_default();
                *e += 1;
            }
            costfreq
        })
}

fn process(minsave: usize, bufin: impl BufRead) -> Result<usize> {
    let g = parser::parse(bufin)?;
    let cheats = cheats_all_calc(&g);
    Ok(cheats
        .into_iter()
        .filter_map(|(save, count)| (save >= minsave).then_some(count))
        .sum())
}

#[test]
fn test_cost_to_end_map() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(g.into_iter().filter(|c| c != &Cell::Wall).count(), 85);
    let end = grid_find(&g, Cell::End);
    let cost_to_end_map = costmap_calc(&g, end);
    let start = grid_find(&g, Cell::Start);
    assert_eq!(cost_to_end_map[start], 84);
    Ok(())
}

#[test]
fn test_cheat_all() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    let cheats = cheats_all_calc(&g);
    assert_eq!(cheats[&50], 32);
    assert_eq!(cheats[&52], 31);
    assert_eq!(cheats[&54], 29);
    assert_eq!(cheats[&56], 39);
    assert_eq!(cheats[&58], 25);
    assert_eq!(cheats[&60], 23);
    assert_eq!(cheats[&62], 20);
    assert_eq!(cheats[&64], 19);
    assert_eq!(cheats[&66], 12);
    assert_eq!(cheats[&68], 14);
    assert_eq!(cheats[&70], 12);
    assert_eq!(cheats[&72], 22);
    assert_eq!(cheats[&74], 4);
    assert_eq!(cheats[&76], 3);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let result = process(50, EXAMPLE.as_bytes())?;
    assert_eq!(
        result,
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    );
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(100, stdin().lock()))
}
