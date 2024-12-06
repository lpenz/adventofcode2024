// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Cell {
    None,
    #[default]
    Wall,
}

pub type Sqrid = sqrid::sqrid_create!(130, 130, false);
pub type Pos = sqrid::pos_create!(Sqrid);

pub mod parser {
    use aoc::parser::*;
    use std::collections::HashSet;

    use super::*;

    fn cell(input: &str) -> IResult<&str, (Cell, bool)> {
        let (input, c) = character::one_of(".#^")(input)?;
        Ok((
            input,
            match c {
                '.' => (Cell::None, false),
                '#' => (Cell::Wall, false),
                '^' => (Cell::None, true),
                _ => panic!("unknown cell {}", c),
            },
        ))
    }

    fn line(input: &str) -> IResult<&str, Vec<(Cell, bool)>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    pub fn parse0(mut bufin: impl BufRead) -> Result<Vec<Vec<(Cell, bool)>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }

    pub fn parse(bufin: impl BufRead) -> Result<(HashSet<Pos>, Pos)> {
        let Ok(grid_raw) = parse0(bufin) else {
            panic!();
        };
        let guard = grid_raw
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, (_, guard))| (guard, (x, y)))
            })
            .find_map(|(g, xy)| g.then_some(xy))
            .map(|(x, y)| Pos::new_unwrap(x as u16, y as u16))
            .unwrap();
        let walls = grid_raw
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, (cell, _))| {
                    if *cell == Cell::Wall {
                        Some(Pos::new_unwrap(x as u16, y as u16))
                    } else {
                        None
                    }
                })
            })
            .collect::<HashSet<_>>();
        Ok((walls, guard))
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 8);
    assert_eq!(input.1 .0, (4, 6));
    Ok(())
}
