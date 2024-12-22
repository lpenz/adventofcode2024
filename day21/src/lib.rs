// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub use sqrid::Dir;
// use sqrid::PosT;

// use std::collections::BinaryHeap;
// use std::collections::HashMap;

mod keypad;
pub use self::keypad::*;

mod numpad;
pub use self::numpad::*;

pub const EXAMPLE: &str = "029A
980A
179A
456A
379A
";

pub mod parser {
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
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 5);
    assert_eq!(input[0].len(), 4);
    Ok(())
}

pub fn full_sequence(robots: usize, seq0: &[NumCell]) -> Vec<KeyCell> {
    let mut seqs = numpad_sequences(seq0);
    for _ in 0..robots {
        let mut newseqs = Vec::<Vec<KeyCell>>::default();
        for seq in &seqs {
            let nextseq = keypad_sequences(seq);
            newseqs.extend(nextseq.into_iter());
        }
        let minlen = newseqs.iter().map(|v| v.len()).min().unwrap();
        seqs = newseqs.into_iter().filter(|v| v.len() == minlen).collect();
    }
    seqs.iter().min_by_key(|v| v.len()).unwrap().to_vec()
}

pub fn full_sequence_reverse(robots: usize, buttons: &[KeyCell]) -> Vec<NumCell> {
    let mut buttons = buttons.to_vec();
    for _ in 0..robots {
        buttons = keypad_use(&buttons);
    }
    numpad_use(&buttons)
}

#[test]
fn test_full_sequence() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(full_sequence(2, &input[0]).len(), 68);
    assert_eq!(full_sequence(2, &input[1]).len(), 60);
    assert_eq!(full_sequence(2, &input[2]).len(), 68);
    assert_eq!(full_sequence(2, &input[3]).len(), 64);
    assert_eq!(full_sequence(2, &input[4]).len(), 64);
    Ok(())
}
