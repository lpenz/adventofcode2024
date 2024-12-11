// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "2333133121414131402\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn digit(input: &str) -> IResult<&str, usize> {
        let (input, dstr) = character::one_of("0123456789")(input)?;
        Ok((input, dstr.to_digit(10).unwrap() as usize))
    }

    fn line(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, digits) = multi::many1(digit)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, digits))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<usize>> {
        aoc::parse_with!(line, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 19);
    Ok(())
}
