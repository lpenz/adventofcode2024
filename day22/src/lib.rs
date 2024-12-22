// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str = "1
10
100
2024
";

pub const EXAMPLE2: &str = "1
2
3
2024
";

pub type Num = i64;

pub fn mix(secret: Num, n: Num) -> Num {
    secret ^ n
}

pub fn prune(secret: Num) -> Num {
    secret % 16777216
}

pub fn evolve(secret: Num) -> Num {
    let mut s = prune(mix(secret, secret * 64));
    s = prune(mix(s, s >> 5));
    prune(mix(s, s * 2048))
}

#[test]
fn test_evolve() {
    assert_eq!(evolve(123), 15887950);
    assert_eq!(evolve(15887950), 16495136);
    assert_eq!(evolve(16495136), 527345);
    assert_eq!(evolve(527345), 704524);
    assert_eq!(evolve(704524), 1553684);
    assert_eq!(evolve(1553684), 12683156);
    assert_eq!(evolve(12683156), 11100544);
    assert_eq!(evolve(11100544), 12249484);
    assert_eq!(evolve(12249484), 7753432);
    assert_eq!(evolve(7753432), 5908254);
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, Num> {
        character::i64(input)
    }

    fn line(input: &str) -> IResult<&str, Num> {
        let (input, num) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Num>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(input.len(), 4);
    Ok(())
}
