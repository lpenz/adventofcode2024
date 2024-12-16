// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub const EXAMPLE1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

pub const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Wall,
    Empty,
    Start,
    End,
}

impl TryFrom<char> for Cell {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            _ => Err(format!("unknown cell {}", c)),
        }
    }
}

pub type Sqrid = sqrid::sqrid_create!(142, 142, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub use sqrid::Dir;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        character::one_of("#.SE")(input).map(|(input, c)| (input, Cell::try_from(c).unwrap()))
    }

    pub fn parse_(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(grid(cell), bufin)
    }

    pub fn parse(bufin: impl BufRead) -> Result<Grid> {
        let vecvec = parse_(bufin)?;
        let mut g = Grid::default();
        g.extend_from_vecvec(vecvec)?;
        Ok(g)
    }
}

#[test]
fn test_a1() -> Result<()> {
    let g = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(calc_best(&g)?, 7036);
    Ok(())
}

#[test]
fn test_a2() -> Result<()> {
    let g = parser::parse(EXAMPLE2.as_bytes())?;
    assert_eq!(calc_best(&g)?, 11048);
    Ok(())
}

pub fn go(g: &Grid, p: Pos, d: Dir) -> Option<Pos> {
    (p + d).ok().filter(|p| g[p] != Cell::Wall)
}

pub fn calc_best(g: &Grid) -> Result<usize> {
    let mut frontier = BinaryHeap::<(Reverse<usize>, Pos, Dir)>::new();
    let pos0 = g
        .iter_pos()
        .find_map(|(p, c)| (*c == Cell::Start).then_some(p))
        .unwrap();
    let dir0 = Dir::E;
    frontier.push((Reverse(0), pos0, dir0));
    let mut visited = HashSet::new();
    while let Some((points0, pos, dir)) = frontier.pop() {
        if g[pos] == Cell::End {
            return Ok(points0.0);
        }
        let key = (pos, dir);
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);
        for turn in [Dir::N, Dir::E, Dir::W] {
            let d = dir + turn;
            if let Some(p) = go(g, pos, d) {
                let points = Reverse(points0.0 + 1 + if turn == Dir::N { 0 } else { 1000 });
                frontier.push((points, p, d));
            }
        }
    }
    Err(eyre!("path not found"))
}
