// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day04::*;

use sqrid::Dir;

fn first(s: &str) -> char {
    s.chars().next().unwrap()
}

fn check(input: &[Vec<char>], xy0: (usize, usize), dir: Dir, s: &str) -> usize {
    if xy0.1 >= input.len() || xy0.0 >= input[xy0.1].len() || input[xy0.1][xy0.0] != first(s) {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }
    if let Ok(xy) = xy0 + dir {
        check(input, xy, dir, &s[1..])
    } else {
        0
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for dir in Dir::iter::<true>() {
                count += check(&input, (x, y), dir, "XMAS");
            }
        }
    }
    Ok(count)
}

#[test]
fn test0() -> Result<()> {
    assert_eq!(process(EXAMPLE0.as_bytes())?, 4);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 18);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
