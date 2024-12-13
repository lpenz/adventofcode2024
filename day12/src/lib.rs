// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC
";

pub const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

pub const EXAMPLE3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

pub const EXAMPLE4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

pub type Cell = char;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        character::none_of("\n")(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, chars) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, chars))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<char>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(input.len(), 4);
    assert_eq!(input[0].len(), 4);
    Ok(())
}
