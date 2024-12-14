// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use itertools::chain;
use std::collections::HashMap;
use std::collections::HashSet;

use day08::*;

fn valid(width: i32, height: i32, xy: &(i32, i32)) -> bool {
    xy.0 >= 0 && xy.0 < width && xy.1 >= 0 && xy.1 < height
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let width = input.len() as i32;
    let height = input[0].len() as i32;
    Ok(input
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter(|(_, cell)| *cell != Cell::Empty)
                .map(move |(x, cell)| (cell, (x as i32, y as i32)))
        })
        .fold(
            HashMap::<Cell, Vec<(i32, i32)>>::default(),
            |mut ants, (cell, pos)| {
                ants.entry(cell).or_default().push(pos);
                ants
            },
        )
        .into_values()
        .flat_map(|xys| {
            xys.iter()
                .flat_map(|&xy1| {
                    xys.iter()
                        .flat_map(move |&xy2| {
                            if xy1 == xy2 {
                                vec![]
                            } else {
                                let dx = xy2.0 - xy1.0;
                                let dy = xy2.1 - xy1.1;
                                let mut xy1 = xy1;
                                let mut xy2 = xy2;
                                chain(
                                    std::iter::from_fn(move || {
                                        xy1.0 += dx;
                                        xy1.1 += dy;
                                        valid(width, height, &xy1).then_some(xy1)
                                    }),
                                    std::iter::from_fn(move || {
                                        xy2.0 += dx;
                                        xy2.1 += dy;
                                        valid(width, height, &xy2).then_some(xy2)
                                    }),
                                )
                                .collect::<Vec<_>>()
                            }
                        })
                        .filter(|xy| valid(width, height, xy))
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 34);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
