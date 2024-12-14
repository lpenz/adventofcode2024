// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

pub type Sqrid = sqrid::sqrid_create!(45, 45, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, u8);
pub use sqrid::Dir;

pub mod parser {
    use aoc::parser::*;

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<u8>>> {
        aoc::parse_with!(grid(digit1), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 8);
    Ok(())
}

pub fn go(g: &Grid, src: Pos, d: Dir) -> Option<Pos> {
    (src + d).ok().filter(|dst| g[dst] == g[src] + 1)
}
