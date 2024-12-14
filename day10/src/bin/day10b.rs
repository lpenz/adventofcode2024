// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day10::*;

use sqrid::postrait::PosT;

fn seek(g: &Grid, pos0: Pos) -> usize {
    if g[pos0] == 9 {
        return 1;
    }
    Dir::iter::<false>()
        .map(|d| {
            if let Some(p) = go(g, pos0, d) {
                seek(g, p)
            } else {
                0
            }
        })
        .sum()
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut g = Grid::repeat(99_u8);
    for (y, line) in input.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            let p = Pos::new_unwrap(x as u16, y as u16);
            g[p] = c;
        }
    }
    Ok(Pos::iter()
        .filter(|p| g[p] == 0)
        .map(|head| seek(&g, head))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 81);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
