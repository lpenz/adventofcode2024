// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Wall => '#',
                Cell::Empty => '.',
                Cell::Start => 'S',
                Cell::End => 'E',
            }
        )
    }
}

pub type Sqrid = sqrid::sqrid_create!(142, 142, false);
//pub type Sqrid = sqrid::sqrid_create!(14, 14, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub type Gridbool = sqrid::gridbool_create!(Sqrid);
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

pub fn grid_find(g: &Grid, cell: Cell) -> Pos {
    g.iter_pos()
        .find_map(|(p, c)| (*c == cell).then_some(p))
        .unwrap()
}

pub fn go(g: &Grid, p: Pos, d: Dir) -> Option<Pos> {
    (p + d).ok().filter(|p| g[p] != Cell::Wall)
}

#[test]
fn test() -> Result<()> {
    let g = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(grid_find(&g, Cell::Start), Pos::new_static::<1, 3>());
    assert_eq!(grid_find(&g, Cell::End), Pos::new_static::<5, 7>());
    Ok(())
}
