// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day22::*;

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

fn process(bufin: impl BufRead) -> Result<Num> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|mut s| {
            for _ in 0..2000 {
                s = evolve(s);
            }
            s
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 37327623);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
