// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "1
10
100
2024
";

pub type Num = u64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, Num> {
        character::u64(input)
    }

    fn line(input: &str) -> IResult<&str, Num> {
        let (input, num) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Num>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 4);
    Ok(())
}
