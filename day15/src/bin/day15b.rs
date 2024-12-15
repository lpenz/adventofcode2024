// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day15::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell2 {
    #[default]
    Wall,
    Empty,
    BoxL,
    BoxR,
    Robot,
}

impl std::fmt::Display for Cell2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell2::Wall => '#',
                Cell2::Empty => '.',
                Cell2::BoxL => '[',
                Cell2::BoxR => ']',
                Cell2::Robot => '@',
            }
        )
    }
}

impl Cell2 {
    pub fn is_box(&self) -> bool {
        self == &Cell2::BoxL || self == &Cell2::BoxR
    }
    pub fn flip_box(self) -> Self {
        match self {
            Cell2::BoxL => Cell2::BoxR,
            Cell2::BoxR => Cell2::BoxL,
            _ => panic!(),
        }
    }
}

pub type Sqrid2 = sqrid::sqrid_create!(150, 150, false);
pub type Pos2 = sqrid::pos_create!(Sqrid2);
pub type Grid2 = sqrid::grid_create!(Sqrid2, Cell2);
pub use sqrid::Dir;

fn get_robot(grid: &Grid2) -> Pos2 {
    grid.iter_pos()
        .find(|(_, c)| **c == Cell2::Robot)
        .unwrap()
        .0
}

fn push_lr(grid: &mut Grid2, orig: Pos2, dir: Dir) -> bool {
    assert!(dir == Dir::E || dir == Dir::W);
    let Ok(dst) = orig + dir else {
        return false;
    };
    if grid[dst] == Cell2::Wall {
        return false;
    }
    if grid[dst] == Cell2::Empty {
        grid[dst] = grid[orig];
        grid[orig] = Cell2::Empty;
        return true;
    }
    // It's a box
    let mut dstbox = dst + dir;
    while dstbox.as_ref().map(|b| grid[b].is_box()) == Ok(true) {
        dstbox = dstbox.unwrap() + dir;
    }
    let Ok(dstbox) = dstbox else {
        return false;
    };
    if grid[dstbox] != Cell2::Empty {
        return false;
    }
    grid[dstbox] = grid[dst].flip_box();
    let mut pos = (dst + dir).unwrap();
    while pos != dstbox {
        grid[pos] = grid[pos].flip_box();
        pos = (pos + dir).unwrap();
    }
    grid[dst] = grid[orig];
    grid[orig] = Cell2::Empty;
    true
}

fn push_ud(grid: &mut Grid2, orig: Pos2, dir: Dir) -> bool {
    assert!(dir == Dir::N || dir == Dir::S);
    let Ok(dst) = orig + dir else {
        return false;
    };
    if grid[orig] == Cell2::Wall || grid[dst] == Cell2::Wall {
        return false;
    }
    if grid[orig] == Cell2::Robot {
        return if grid[dst] == Cell2::Empty {
            grid[dst] = grid[orig];
            grid[orig] = Cell2::Empty;
            true
        } else {
            // dst is a box
            let mut g = *grid;
            if push_ud(&mut g, dst, dir) {
                *grid = g;
                grid[dst] = grid[orig];
                grid[orig] = Cell2::Empty;
                true
            } else {
                false
            }
        };
    }
    // We are a box
    let orig2 = match grid[orig] {
        Cell2::BoxL => (orig + Dir::E).unwrap(),
        Cell2::BoxR => (orig + Dir::W).unwrap(),
        _ => panic!("{:?} is not a box", grid[orig]),
    };
    let dst2 = match grid[orig] {
        Cell2::BoxL => (dst + Dir::E).unwrap(),
        Cell2::BoxR => (dst + Dir::W).unwrap(),
        _ => panic!("{:?} is not a box", grid[orig]),
    };
    if grid[dst] == Cell2::Empty && grid[dst2] == Cell2::Empty {
        grid[dst] = grid[orig];
        grid[dst2] = grid[orig2];
        grid[orig] = Cell2::Empty;
        grid[orig2] = Cell2::Empty;
        return true;
    }
    let mut g = *grid;
    if g[dst] != Cell2::Empty && !push_ud(&mut g, dst, dir) {
        return false;
    }
    if g[dst2] != Cell2::Empty && !push_ud(&mut g, dst2, dir) {
        return false;
    }
    *grid = g;
    grid[dst] = grid[orig];
    grid[dst2] = grid[orig2];
    grid[orig] = Cell2::Empty;
    grid[orig2] = Cell2::Empty;
    true
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (gridvec, dirs) = parser::parse(bufin)?;
    let mut grid = Grid2::default();
    for (y, line) in gridvec.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            let p1 = Pos2::new_unwrap(2 * x as u16, y as u16);
            let p2 = Pos2::new_unwrap(2 * x as u16 + 1, y as u16);
            grid[p1] = match c {
                Cell::Wall => Cell2::Wall,
                Cell::Empty => Cell2::Empty,
                Cell::Box => Cell2::BoxL,
                Cell::Robot => Cell2::Robot,
            };
            grid[p2] = match c {
                Cell::Wall => Cell2::Wall,
                Cell::Empty => Cell2::Empty,
                Cell::Box => Cell2::BoxR,
                Cell::Robot => Cell2::Empty,
            };
        }
    }
    for dir in dirs.into_iter() {
        let robot = get_robot(&grid);
        if dir == Dir::E || dir == Dir::W {
            push_lr(&mut grid, robot, dir);
        } else {
            push_ud(&mut grid, robot, dir);
        }
    }
    Ok(grid
        .iter_pos()
        .map(|(p, c)| {
            if c == &Cell2::BoxL {
                100 * p.y() as usize + p.x() as usize
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test3() -> Result<()> {
    assert_eq!(process(EXAMPLE3.as_bytes())?, 618);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 9021);
    Ok(())
}

#[test]
fn test_push_r() -> Result<()> {
    let mut g = Grid2::default();
    g.extend_from_vecvec(vec![vec![
        Cell2::Robot,
        Cell2::BoxL,
        Cell2::BoxR,
        Cell2::BoxL,
        Cell2::BoxR,
        Cell2::Empty,
        Cell2::Empty,
    ]])?;
    let robot = get_robot(&g);
    assert!(push_lr(&mut g, robot, Dir::E));
    let robot = get_robot(&g);
    assert!(push_lr(&mut g, robot, Dir::E));
    let robot = get_robot(&g);
    assert!(!push_lr(&mut g, robot, Dir::E));
    Ok(())
}

#[test]
fn test_push_l() -> Result<()> {
    let mut g = Grid2::default();
    g.extend_from_vecvec(vec![vec![
        Cell2::Empty,
        Cell2::Empty,
        Cell2::BoxL,
        Cell2::BoxR,
        Cell2::BoxL,
        Cell2::BoxR,
        Cell2::Robot,
    ]])?;
    let robot = get_robot(&g);
    assert!(push_lr(&mut g, robot, Dir::W));
    let robot = get_robot(&g);
    assert!(push_lr(&mut g, robot, Dir::W));
    let robot = get_robot(&g);
    assert!(!push_lr(&mut g, robot, Dir::W));
    Ok(())
}

#[test]
fn test_push_d() -> Result<()> {
    let mut g = Grid2::default();
    g.extend_from_vecvec(vec![
        vec![
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Robot,
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Empty,
        ],
        vec![
            Cell2::Empty,
            Cell2::Empty,
            Cell2::BoxL,
            Cell2::BoxR,
            Cell2::Empty,
            Cell2::Empty,
        ],
        vec![
            Cell2::Empty,
            Cell2::BoxL,
            Cell2::BoxR,
            Cell2::BoxL,
            Cell2::BoxR,
            Cell2::Empty,
        ],
        vec![
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Empty,
            Cell2::Empty,
        ],
    ])?;
    let robot = get_robot(&g);
    assert!(push_ud(&mut g, robot, Dir::S));
    let robot = get_robot(&g);
    assert!(!push_ud(&mut g, robot, Dir::S));
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
