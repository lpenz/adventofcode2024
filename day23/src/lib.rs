// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cpu(pub copstr::Str<2>);

impl Cpu {
    pub fn prefix_t(&self) -> bool {
        self.0.as_str().starts_with('t')
    }
}

pub type Connection = (Cpu, Cpu);

pub fn connect(cpu1: Cpu, cpu2: Cpu) -> Connection {
    (std::cmp::min(cpu1, cpu2), std::cmp::max(cpu1, cpu2))
}

pub const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cpu(input: &str) -> IResult<&str, Cpu> {
        let (input, name) = character::alpha1(input)?;
        Ok((input, Cpu(name.try_into().unwrap())))
    }

    fn line(input: &str) -> IResult<&str, Connection> {
        let (input, cpu1) = cpu(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, cpu2) = cpu(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, connect(cpu1, cpu2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Connection>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 32);
    Ok(())
}
