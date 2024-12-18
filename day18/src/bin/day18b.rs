// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day18::*;

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

fn go(gb: &Gridbool, size: u16, p: Pos, d: Dir) -> Option<Pos> {
    (p + d)
        .ok()
        .filter(|p| !gb.get(p) && p.x() < size && p.y() < size)
}

fn process(size: u16, bufin: impl BufRead) -> Result<Pos> {
    let input = parser::parse(bufin)?;
    let target = Pos::new_unwrap(size - 1, size - 1);
    input
        .into_iter()
        .scan((Pos::TOP_LEFT, Gridbool::default()), |(_, gb), p| {
            gb.set_t(&p);
            Some((p, *gb))
        })
        .par_bridge()
        .find_first(|(_, gb)| {
            let pathopt = Sqrid::astar_path(|p, d| go(gb, size, p, d), &Pos::TOP_LEFT, &target);
            pathopt.is_err()
        })
        .map(|(p, _)| p)
        .ok_or_else(|| eyre!("path never blocked"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(7, EXAMPLE.as_bytes())?, Pos::new_unwrap(6, 1));
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(71, stdin().lock()))
}
