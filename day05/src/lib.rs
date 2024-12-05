// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

pub type Rule = (u32, u32);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, u32> {
        character::u32(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, n1) = num(input)?;
        let (input, _) = tag("|")(input)?;
        let (input, n2) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (n1, n2)))
    }

    fn update(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, update) = multi::separated_list1(tag(","), num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, update))
    }

    fn both(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<u32>>)> {
        let (input, rules) = multi::many1(rule)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, updates) = multi::many1(update)(input)?;
        Ok((input, (rules, updates)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Rule>, Vec<Vec<u32>>)> {
        aoc::parse_with!(both, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 21);
    assert_eq!(input.1.len(), 6);
    Ok(())
}
