// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n";

pub const EXAMPLE2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n";

#[derive(Debug)]
pub enum Instr {
    Do,
    Dont,
    Mul(u64, u64),
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn parse_do(input: &str) -> IResult<&str, Option<Instr>> {
        let (input, _) = tag("do()")(input)?;
        Ok((input, Some(Instr::Do)))
    }

    fn parse_dont(input: &str) -> IResult<&str, Option<Instr>> {
        let (input, _) = tag("don't()")(input)?;
        Ok((input, Some(Instr::Dont)))
    }

    fn parse_mul(input: &str) -> IResult<&str, Option<Instr>> {
        let (input, _) = tag("mul(")(input)?;
        let (input, n1) = character::u64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, n2) = character::u64(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, Some(Instr::Mul(n1, n2))))
    }

    fn parse_corruption(input: &str) -> IResult<&str, Option<Instr>> {
        let (input, _) = character::anychar(input)?;
        Ok((input, None))
    }

    fn token(input: &str) -> IResult<&str, Option<Instr>> {
        branch::alt((parse_do, parse_dont, parse_mul, parse_corruption))(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Instr>> {
        aoc::parse_with!(multi::many1(token), bufin).map(|opts| {
            opts.into_iter()
                .filter(|opt| opt.is_some())
                .collect::<Option<Vec<Instr>>>()
                .unwrap()
        })
    }
}

#[test]
fn test1() -> Result<()> {
    let input = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(input.len(), 4);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input = parser::parse(EXAMPLE2.as_bytes())?;
    assert_eq!(input.len(), 6);
    Ok(())
}
