// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use aoc::parser::*;

use super::*;

fn num(input: &str) -> IResult<&str, NumCell> {
    let (input, c) = character::one_of("0123456789A")(input)?;
    Ok((input, c.into()))
}

fn line(input: &str) -> IResult<&str, Vec<NumCell>> {
    let (input, numcells) = multi::many1(num)(input)?;
    let (input, _) = character::newline(input)?;
    Ok((input, numcells))
}

pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<NumCell>>> {
    aoc::parse_with!(multi::many1(line), bufin)
}

fn key(input: &str) -> IResult<&str, KeyCell> {
    let (input, c) = character::one_of("<>^vA")(input)?;
    Ok((input, c.into()))
}

pub fn parse_keys(mut bufin: impl BufRead) -> Result<Vec<KeyCell>> {
    aoc::parse_with!(multi::many1(key), bufin)
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 5);
    assert_eq!(input[0].len(), 4);
    Ok(())
}
