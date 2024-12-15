// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day15::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (gridvec, dirs) = parser::parse(bufin)?;
    let mut grid = Grid::default();
    grid.extend_from_vecvec(gridvec)?;
    let mut robot = grid.iter_pos().find(|(_, c)| **c == Cell::Robot).unwrap().0;
    for d in dirs.into_iter() {
        grid[robot] = Cell::Empty;
        if let Ok(dst) = robot + d {
            match grid[dst] {
                Cell::Empty => {
                    robot = dst;
                }
                Cell::Wall => {}
                Cell::Box => {
                    let mut dstbox = dst + d;
                    while dstbox.as_ref().map(|b| grid[b]) == Ok(Cell::Box) {
                        dstbox = dstbox.unwrap() + d;
                    }
                    if let Ok(dstbox) = dstbox {
                        if grid[dstbox] == Cell::Empty {
                            grid[dstbox] = Cell::Box;
                            grid[dst] = Cell::Empty;
                            robot = dst;
                        }
                    }
                }
                _ => panic!(),
            }
        }
        grid[robot] = Cell::Robot;
    }
    Ok(grid
        .iter_pos()
        .map(|(p, c)| {
            if c == &Cell::Box {
                100 * p.y() as usize + p.x() as usize
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 2028);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 10092);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
