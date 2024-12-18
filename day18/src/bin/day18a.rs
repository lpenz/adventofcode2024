// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day18::*;

fn go(gb: &Gridbool, size: u16, p: Pos, d: Dir) -> Option<(Pos, usize)> {
    (p + d)
        .ok()
        .filter(|p| !gb.get(p) && p.x() < size && p.y() < size)
        .map(|p| (p, 1))
}

fn process(falls: usize, size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let gb = input.into_iter().take(falls).collect::<Gridbool>();
    let target = Pos::new_unwrap(size - 1, size - 1);
    if let Ok(path) = Sqrid::ucs_path(|p, d| go(&gb, size, p, d), &Pos::TOP_LEFT, &target) {
        Ok(path.len())
    } else {
        Err(eyre!("could not find path"))
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(12, 7, EXAMPLE.as_bytes())?, 22);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(1024, 71, stdin().lock()))
}
