// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

pub type Sqrid = sqrid::sqrid_create!(70, 70, false);
// pub type Sqrid = sqrid::sqrid_create!(6, 6, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Gridbool = sqrid::gridbool_create!(Sqrid);
pub type Dir = sqrid::Dir;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn line(input: &str) -> IResult<&str, Pos> {
        let (input, x) = character::u16(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = character::u16(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Pos::new_unwrap(x, y)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Pos>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 25);
    Ok(())
}
