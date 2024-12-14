// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::cmp::Ordering;

use day14::*;

fn process(width: i32, height: i32, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|bot| Robot {
            p: (
                (bot.p.0 + 100 * bot.v.0).rem_euclid(width),
                (bot.p.1 + 100 * bot.v.1).rem_euclid(height),
            ),
            v: bot.v,
        })
        .fold(vec![0, 0, 0, 0], |mut quads, bot| {
            match (bot.p.0.cmp(&(width / 2)), bot.p.1.cmp(&(height / 2))) {
                (Ordering::Less, Ordering::Less) => {
                    quads[0] += 1;
                }
                (Ordering::Greater, Ordering::Less) => {
                    quads[1] += 1;
                }
                (Ordering::Less, Ordering::Greater) => {
                    quads[2] += 1;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    quads[3] += 1;
                }
                (_, _) => {}
            }
            quads
        })
        .into_iter()
        .product())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(11, 7, EXAMPLE.as_bytes())?, 12);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(101, 103, stdin().lock()))
}
