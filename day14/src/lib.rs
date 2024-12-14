// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[derive(Debug)]
pub struct Robot {
    pub p: (i32, i32),
    pub v: (i32, i32),
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, i32> {
        character::i32(input)
    }

    fn line(input: &str) -> IResult<&str, Robot> {
        let (input, _) = tag("p=")(input)?;
        let (input, x) = num(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = num(input)?;
        let (input, _) = tag(" v=")(input)?;
        let (input, vx) = num(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, vy) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            Robot {
                p: (x, y),
                v: (vx, vy),
            },
        ))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Robot>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 12);
    Ok(())
}
