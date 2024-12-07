// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

pub type N = u64;
pub type Equation = (N, Vec<N>);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, N> {
        character::u64(input)
    }

    fn line(input: &str) -> IResult<&str, Equation> {
        let (input, result) = num(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, operands) = multi::separated_list1(tag(" "), num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (result, operands)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Equation>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 9);
    Ok(())
}
