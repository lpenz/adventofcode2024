// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day10::*;

use sqrid::postrait::PosT;

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
        .map(|head| {
            Sqrid::bf_iter(|p, d| go(&g, p, d), &head)
                .flatten()
                .filter(|(p, _)| g[p] == 9)
                .count()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 36);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
