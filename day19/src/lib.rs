// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}

impl TryFrom<char> for Color {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Color::W),
            'u' => Ok(Color::U),
            'b' => Ok(Color::B),
            'r' => Ok(Color::R),
            'g' => Ok(Color::G),
            _ => Err(format!("invalid color {}", c)),
        }
    }
}

pub type Towel = Vec<Color>;

pub type Design = Vec<Color>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn color(input: &str) -> IResult<&str, Color> {
        let (input, c) = character::one_of("wubrg")(input)?;
        Ok((input, Color::try_from(c).expect("invalid color")))
    }

    fn towels(input: &str) -> IResult<&str, Vec<Towel>> {
        let (input, towels) = multi::separated_list1(tag(", "), multi::many1(color))(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, towels))
    }

    fn design(input: &str) -> IResult<&str, Design> {
        let (input, design) = multi::many1(color)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, design))
    }

    fn all(input: &str) -> IResult<&str, (Vec<Towel>, Vec<Design>)> {
        let (input, towels) = towels(input)?;
        let (input, _) = character::newline(input)?;
        let (input, designs) = multi::many1(design)(input)?;
        Ok((input, (towels, designs)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Towel>, Vec<Design>)> {
        aoc::parse_with!(all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 8);
    assert_eq!(input.1.len(), 8);
    Ok(())
}
