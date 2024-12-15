// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<
";

pub const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[default]
    Wall,
    Empty,
    Box,
    Robot,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Wall => '#',
                Cell::Empty => '.',
                Cell::Box => 'O',
                Cell::Robot => '@',
            }
        )
    }
}

pub type Sqrid = sqrid::sqrid_create!(50, 50, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub use sqrid::Dir;

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'O' => Cell::Box,
            '@' => Cell::Robot,
            _ => panic!("invalid character {}", c),
        }
    }
}

pub mod parser {
    use aoc::parser::*;
    use sqrid::Dir;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        character::one_of("#.O@")(input).map(|(input, c)| (input, Cell::from(c)))
    }

    fn nl(input: &str) -> IResult<&str, Option<Dir>> {
        let (input, _) = tag("\n")(input)?;
        Ok((input, None))
    }

    fn dir(input: &str) -> IResult<&str, Option<Dir>> {
        let (input, dirchar) = character::one_of("<>^v")(input)?;
        Ok((input, Some(Dir::try_from(dirchar).unwrap())))
    }

    fn griddirs(input: &str) -> IResult<&str, (Vec<Vec<Cell>>, Vec<Dir>)> {
        let (input, grid) = grid(cell)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, diropts) = multi::many1(branch::alt((dir, nl)))(input)?;
        Ok((input, (grid, diropts.into_iter().flatten().collect())))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Vec<Cell>>, Vec<Dir>)> {
        aoc::parse_with!(griddirs, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let (grid, dirs) = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(grid.len(), 8);
    assert_eq!(grid[0].len(), 8);
    assert_eq!(dirs.len(), 15);
    Ok(())
}
