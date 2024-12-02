// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, i32> {
        character::i32(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<i32>> {
        let (input, nums) = multi::separated_list1(character::space1, num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, nums))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<i32>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 6);
    assert_eq!(input[0].len(), 5);
    Ok(())
}
