// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day12::*;

use sqrid::Dir;

use std::collections::HashMap;
use std::collections::HashSet;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let plantmap = input
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>();
    let mut visited = HashSet::<(i32, i32)>::default();
    let mut regions = vec![];
    // Collect the regions
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let xy0 = (x as i32, y as i32);
            if visited.contains(&xy0) {
                continue;
            }
            let mut region = HashSet::<(i32, i32)>::default();
            region.insert(xy0);
            let mut pending = vec![xy0];
            while let Some(xy) = pending.pop() {
                if visited.contains(&xy) {
                    continue;
                }
                visited.insert(xy);
                for d in Dir::iter::<false>() {
                    if let Ok(new_xy) = xy + d {
                        if plantmap.get(&new_xy) == Some(c) {
                            region.insert(new_xy);
                            pending.push(new_xy);
                        }
                    }
                }
            }
            regions.push((c, region));
        }
    }
    // Calculate each area, perimeter and use them to calculate the final result.
    Ok(regions
        .into_iter()
        .map(|(_, coords)| {
            let area = coords.len();
            let perimeter: usize = coords
                .iter()
                .flat_map(|xy| {
                    Dir::iter::<false>().map(|d| {
                        let xy = *xy;
                        if let Ok(neigh) = xy + d {
                            if !coords.clone().contains(&neigh) {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    })
                })
                .sum();
            area * perimeter
        })
        .sum())
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 140);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 772);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
