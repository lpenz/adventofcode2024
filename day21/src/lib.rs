// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use cached::proc_macro::cached;
use cached::SizedCache;

pub use sqrid::Dir;

mod keypad;
pub use self::keypad::*;

mod numpad;
pub use self::numpad::*;

pub mod parser;

pub const EXAMPLE: &str = "029A
980A
179A
456A
379A
";

pub fn numpad_sequence_len(robots: usize, numcells: &[NumCell]) -> usize {
    let numpad = Numpad::default();
    let keypad = Keypad::default();
    numcells
        .iter()
        .fold((0, NumCell::A), |(mut len, lastcell), cell| {
            len += numpad
                .paths_get(&lastcell, cell)
                .map(|seq| keypad_sequence_len(&keypad, robots, seq))
                .min()
                .unwrap();
            (len, *cell)
        })
        .0
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(10000) }",
    convert = r#"{ format!("{}~{:?}", robots, keycells) }"#
)]
pub fn keypad_sequence_len(keypad: &Keypad, robots: usize, keycells: &[KeyCell]) -> usize {
    if robots == 0 {
        return keycells.len();
    }
    keycells
        .iter()
        .fold((0, KeyCell::A), |(mut len, lastcell), cell| {
            len += keypad
                .paths_get(&lastcell, cell)
                .map(|seq| keypad_sequence_len(keypad, robots - 1, seq))
                .min()
                .unwrap();
            (len, *cell)
        })
        .0
}

#[test]
fn test_full_sequence() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(numpad_sequence_len(2, &input[0]), 68);
    assert_eq!(numpad_sequence_len(2, &input[1]), 60);
    assert_eq!(numpad_sequence_len(2, &input[2]), 68);
    assert_eq!(numpad_sequence_len(2, &input[3]), 64);
    assert_eq!(numpad_sequence_len(2, &input[4]), 64);
    Ok(())
}

pub fn numericpart_calc(code: &[NumCell]) -> usize {
    code.iter().fold(0, |acc, n| {
        if let Ok(n) = usize::try_from(*n) {
            acc * 10 + n
        } else {
            acc
        }
    })
}

#[test]
fn test_numericpart_calc() {
    assert_eq!(
        numericpart_calc(&vec![
            NumCell::Num(0),
            NumCell::Num(2),
            NumCell::Num(9),
            NumCell::A
        ]),
        29
    );
}
