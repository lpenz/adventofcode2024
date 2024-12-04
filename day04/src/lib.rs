// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use sqrid::Dir;

pub const EXAMPLE0: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....
";

pub const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

pub mod parser {
    use aoc::parser::*;

    fn line(input: &str) -> IResult<&str, Vec<char>> {
        let (input, chars) = multi::many1(character::none_of("\n"))(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, chars))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<char>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 10);
    Ok(())
}

pub fn first(s: &str) -> char {
    s.chars().next().unwrap()
}

pub fn check(input: &[Vec<char>], xy0: (usize, usize), dir: Dir, s: &str) -> usize {
    if xy0.1 >= input.len() || xy0.0 >= input[xy0.1].len() || input[xy0.1][xy0.0] != first(s) {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }
    if let Ok(xy) = xy0 + dir {
        check(input, xy, dir, &s[1..])
    } else {
        0
    }
}
