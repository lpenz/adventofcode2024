// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use cached::proc_macro::cached;

pub use aoc::*;

pub const EXAMPLE: &str = "0 1 10 99 999\n";

pub type Stone = u64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn line(input: &str) -> IResult<&str, Vec<Stone>> {
        let (input, num) = multi::separated_list1(tag(" "), character::u64)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Stone>> {
        aoc::parse_with!(line, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 5);
    Ok(())
}

#[cached]
fn blinks(num: usize, value: Stone) -> usize {
    if num == 0 {
        1
    } else if value == 0 {
        blinks(num - 1, 1)
    } else {
        let s = format!("{}", value);
        if s.len() % 2 == 0 {
            let half = s.len() / 2;
            blinks(num - 1, s[0..half].parse::<Stone>().unwrap())
                + blinks(num - 1, s[half..].parse::<Stone>().unwrap())
        } else {
            blinks(num - 1, value * 2024)
        }
    }
}

pub fn solve(num: usize, stones: &[Stone]) -> usize {
    stones.iter().map(|s| blinks(num, *s)).sum()
}
