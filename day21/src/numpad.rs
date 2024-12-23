// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use sqrid::Dir;
use sqrid::PosT;

use std::sync::OnceLock;

use std::collections::HashMap;

pub use super::keypad::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumCell {
    #[default]
    Invalid,
    Num(u8),
    A,
}

impl From<char> for NumCell {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => NumCell::Num(c.to_digit(10).unwrap() as u8),
            'A' => NumCell::A,
            _ => panic!("unknown cell {}", c),
        }
    }
}

impl TryFrom<NumCell> for usize {
    type Error = String;
    fn try_from(c: NumCell) -> Result<Self, Self::Error> {
        match c {
            NumCell::Num(n) => Ok(n as usize),
            _ => Err(format!("{:?} is not a number", c)),
        }
    }
}

impl std::fmt::Display for NumCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NumCell::A => "A".to_string(),
                NumCell::Num(n) => format!("{}", n),
                _ => panic!("can't express invalid numcell"),
            }
        )
    }
}

pub type SqridNum = sqrid::sqrid_create!(2, 3, false);
pub type PosNum = sqrid::pos_create!(SqridNum);
pub type GridNum = sqrid::grid_create!(SqridNum, NumCell);

pub const NUMPAD_POS_START: PosNum = PosNum::new_static::<2, 3>();

#[derive(Clone)]
pub struct Numpad {
    pub grid: GridNum,
    pub paths: HashMap<(NumCell, NumCell), Vec<Vec<KeyCell>>>,
}

impl Default for Numpad {
    fn default() -> Self {
        Self::new()
    }
}

impl Numpad {
    pub fn new() -> Self {
        let grid = [
            NumCell::Num(7),
            NumCell::Num(8),
            NumCell::Num(9),
            NumCell::Num(4),
            NumCell::Num(5),
            NumCell::Num(6),
            NumCell::Num(1),
            NumCell::Num(2),
            NumCell::Num(3),
            NumCell::Invalid,
            NumCell::Num(0),
            NumCell::A,
        ]
        .into_iter()
        .collect::<GridNum>();
        Self {
            grid,
            paths: numpad_bestpaths(&grid),
        }
    }

    pub fn press(&self, buttons: &[KeyCell]) -> Vec<NumCell> {
        buttons
            .iter()
            .fold((NUMPAD_POS_START, vec![]), |(mut pos, mut ret), button| {
                match button {
                    KeyCell::Invalid => panic!("went over invalid!"),
                    KeyCell::D(dir) => pos = (pos + *dir).expect("invalid direction"),
                    KeyCell::A => ret.push(self.grid[pos]),
                }
                (pos, ret)
            })
            .1
    }

    pub fn paths_get(&self, start: &NumCell, end: &NumCell) -> impl Iterator<Item = &Vec<KeyCell>> {
        self.paths.get(&(*start, *end)).unwrap().iter()
    }
}

pub fn numseq2str(seq: &[NumCell]) -> String {
    let mut s = String::new();
    for d in seq {
        s.push_str(&format!("{}", d));
    }
    s
}

fn numpad_dfs(
    g: &GridNum,
    src: PosNum,
    pos0: PosNum,
    path: &mut Vec<KeyCell>,
    dpath: &mut Vec<Dir>,
    bestpaths: &mut HashMap<(NumCell, NumCell), Vec<Vec<KeyCell>>>,
) {
    for dir in Dir::iter::<false>() {
        if dpath.contains(&dir) && dpath[dpath.len() - 1] != dir {
            continue;
        }
        let Some(pos) = (pos0 + dir)
            .ok()
            .filter(|p| *p != src && g[p] != NumCell::Invalid)
        else {
            continue;
        };
        path.push(KeyCell::D(dir));
        dpath.push(dir);
        let e = bestpaths.entry((g[src], g[pos])).or_default();
        if e.is_empty() || path.len() <= e[0].len() {
            path.push(KeyCell::A);
            if e.is_empty() || path.len() < e[0].len() {
                *e = vec![path.clone()];
            } else if path.len() == e[0].len() {
                e.push(path.clone());
            }
            path.pop();
            numpad_dfs(g, src, pos, path, dpath, bestpaths);
        }
        path.pop();
        dpath.pop();
    }
}

fn numpad_bestpaths(g: &GridNum) -> HashMap<(NumCell, NumCell), Vec<Vec<KeyCell>>> {
    let mut bestpaths = HashMap::<(NumCell, NumCell), Vec<Vec<KeyCell>>>::new();
    for p in PosNum::iter() {
        if g[p] == NumCell::Invalid {
            continue;
        }
        bestpaths.insert((g[p], g[p]), vec![]);
        let mut path = vec![];
        let mut dpath = vec![];
        numpad_dfs(g, p, p, &mut path, &mut dpath, &mut bestpaths);
    }
    bestpaths
}

pub fn numpad_get() -> &'static Numpad {
    static MEM: OnceLock<Numpad> = OnceLock::new();
    MEM.get_or_init(Numpad::default)
}

#[test]
fn test_numpad() {
    let numpad = numpad_get();
    assert_eq!(numpad.paths[&(NumCell::A, NumCell::Num(7))].len(), 1);
    assert_eq!(numpad.paths[&(NumCell::Num(3), NumCell::Num(7))].len(), 2);
}
