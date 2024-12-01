// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, i32> {
        character::i32(input)
    }

    fn line(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, n1) = num(input)?;
        let (input, _) = character::space1(input)?;
        let (input, n2) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (n1, n2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(i32, i32)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 6);
    Ok(())
}
