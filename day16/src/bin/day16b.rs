// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day16::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Node = (Pos, Dir);
type Camefrom = HashMap<Node, (usize, Vec<Node>)>;

pub fn collect_paths(
    g: &Grid,
    camefrom: &Camefrom,
    path: &mut Vec<Node>,
    allpaths: &mut Vec<Vec<Pos>>,
) {
    let (pos, dir) = path[path.len() - 1];
    if g[pos] == Cell::Start {
        allpaths.push(path.iter().map(|(p, _)| *p).collect::<Vec<_>>());
        return;
    }
    let nodes = &camefrom.get(&(pos, dir)).unwrap().1;
    for node in nodes {
        path.push(*node);
        collect_paths(g, camefrom, path, allpaths);
        path.pop();
    }
}

pub fn allpaths_best(g: &Grid, target: usize) -> Result<Vec<Vec<Pos>>> {
    let mut frontier = BinaryHeap::<(Reverse<usize>, Node)>::new();
    let node0 = (grid_find(g, Cell::Start), Dir::E);
    frontier.push((Reverse(0), node0));
    let mut camefrom = Camefrom::default();
    camefrom.insert(node0, (0, vec![]));
    let end = grid_find(g, Cell::End);
    while let Some((points0, node)) = frontier.pop() {
        let (pos, dir) = node;
        if points0.0 > target {
            break;
        }
        if points0.0 == target && pos == end {
            continue;
        }
        for turn in [Dir::N, Dir::E, Dir::W] {
            let points = points0.0 + 1 + if turn == Dir::N { 0 } else { 1000 };
            let d = dir + turn;
            if let Some(p) = go(g, pos, d) {
                let n = (p, d);
                let e = camefrom.entry(n).or_insert((usize::MAX, vec![]));
                let oldpoints: usize = e.0;
                if points <= oldpoints {
                    frontier.push((Reverse(points), n));
                    if points < oldpoints {
                        *e = (points, vec![node]);
                    } else if !e.1.contains(&node) {
                        e.1.push(node);
                    }
                }
            }
        }
    }
    let mut allpaths: Vec<Vec<Pos>> = vec![];
    let mut path = camefrom
        .iter()
        .filter_map(|((pos, dir), _)| (*pos == end).then_some((*pos, *dir)))
        .collect::<Vec<_>>();
    collect_paths(g, &camefrom, &mut path, &mut allpaths);
    Ok(allpaths)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let g = parser::parse(bufin)?;
    let points = calc_best(&g)?;
    let all = allpaths_best(&g, points)?;
    let tiles = all
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect::<HashSet<_>>();
    Ok(tiles.len())
}

#[test]
fn test_b1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 45);
    Ok(())
}

#[test]
fn test_b2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 64);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
