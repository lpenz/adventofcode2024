// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;

use day14::*;

fn disp(width: i32, height: i32, bots: &[Robot]) {
    let bots = bots
        .iter()
        .map(|bot| (bot.p.0, bot.p.1))
        .collect::<HashSet<(i32, i32)>>();
    for y in 0..height {
        for x in 0..width {
            print!("{}", if bots.contains(&(x, y)) { "X" } else { " " });
        }
        println!();
    }
}

fn next_second(width: i32, height: i32, bots: &mut [Robot]) {
    for bot in bots.iter_mut() {
        let x = (bot.p.0 + bot.v.0).rem_euclid(width);
        let y = (bot.p.1 + bot.v.1).rem_euclid(height);
        bot.p = (x, y);
    }
}

fn process(width: i32, height: i32, bufin: impl BufRead) -> Result<usize> {
    // Did this one "manually"
    let mut bots = parser::parse(bufin)?;
    let found = 7687;
    for _ in 0..found {
        next_second(width, height, &mut bots);
    }
    println!("{}", found);
    disp(width, height, &bots);
    Ok(0)
}

fn main() -> Result<()> {
    do_main(|| process(101, 103, stdin().lock()))
}
