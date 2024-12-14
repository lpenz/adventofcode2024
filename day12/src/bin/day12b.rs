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
    // Calculate each area and sides and use them to calculate the final result.
    Ok(regions
        .into_iter()
        .map(|(_c, coords)| {
            let area = coords.len();
            let mut visited = HashSet::<((i32, i32), Dir)>::default();
            let mut sides = 0;
            for xy in coords.iter() {
                for d in Dir::iter::<false>() {
                    let outside = (xy + d).unwrap();
                    if visited.contains(&(outside, d)) || coords.contains(&outside) {
                        continue;
                    }
                    sides += 1;
                    for turn in [Dir::E, Dir::W] {
                        let mut inside = *xy;
                        let mut outside = (xy + d).unwrap();
                        let rund = d + turn;
                        loop {
                            outside = (outside + rund).unwrap();
                            inside = (inside + rund).unwrap();
                            if !coords.contains(&inside) || coords.contains(&outside) {
                                break;
                            }
                            visited.insert((outside, d));
                        }
                    }
                }
            }
            area * sides
        })
        .sum())
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 80);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 436);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    assert_eq!(process(EXAMPLE3.as_bytes())?, 236);
    Ok(())
}

#[test]
fn test4() -> Result<()> {
    assert_eq!(process(EXAMPLE4.as_bytes())?, 368);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
