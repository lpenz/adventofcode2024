// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;
pub use num::complex::Complex;

pub const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

pub type Num = i32;
pub type XY = Complex<i32>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, Num> {
        character::i32(input)
    }

    fn button_a(input: &str) -> IResult<&str, XY> {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, x) = num(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Complex::new(x, y)))
    }

    fn button_b(input: &str) -> IResult<&str, XY> {
        let (input, _) = tag("Button B: X+")(input)?;
        let (input, x) = num(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, y) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Complex::new(x, y)))
    }

    fn prize(input: &str) -> IResult<&str, XY> {
        let (input, _) = tag("Prize: X=")(input)?;
        let (input, x) = num(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, y) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Complex::new(x, y)))
    }

    fn entry(input: &str) -> IResult<&str, (XY, XY, XY)> {
        let (input, ba) = button_a(input)?;
        let (input, bb) = button_b(input)?;
        let (input, prize) = prize(input)?;
        Ok((input, (ba, bb, prize)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(XY, XY, XY)>> {
        aoc::parse_with!(multi::separated_list1(character::newline, entry), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 4);
    Ok(())
}
