// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day04::*;

use sqrid::Dir;

fn jump(xy: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    (xy + dir).ok().and_then(|xy| (xy + dir).ok())
}

fn xcheck(input: &[Vec<char>], xy0: (usize, usize), dir0: Dir, dirjump: Dir, dir2: Dir) -> usize {
    if check(input, xy0, dir0, "MAS") > 0 {
        if let Some(xy2) = jump(xy0, dirjump) {
            if check(input, xy2, dir2, "MAS") > 0 {
                return 1;
            }
        }
    }
    0
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'M' {
                continue;
            }
            let xy = (x, y);
            count += xcheck(&input, xy, Dir::SE, Dir::E, Dir::SW);
            count += xcheck(&input, xy, Dir::SE, Dir::S, Dir::NE);
            count += xcheck(&input, xy, Dir::NW, Dir::W, Dir::NE);
            count += xcheck(&input, xy, Dir::NW, Dir::N, Dir::SW);
        }
    }
    Ok(count)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 9);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
