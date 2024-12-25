// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day25::*;

use rayon::prelude::*;

fn fits(lock: &Lock, key: &Key) -> bool {
    lock.0
        .par_iter()
        .enumerate()
        .all(|(i, pin)| pin + key.0[i] <= 6)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (locks, keys) = parser::parse(bufin)?;
    Ok(locks
        .into_par_iter()
        .map(|lock| keys.iter().filter(|key| fits(&lock, key)).count())
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 3);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
