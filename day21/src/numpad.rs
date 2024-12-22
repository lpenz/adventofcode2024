// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use sqrid::Dir;
use sqrid::PosT;

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

pub fn numpad_get() -> GridNum {
    [
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
    .collect::<GridNum>()
}

pub fn numseq2str(seq: &[NumCell]) -> String {
    let mut s = String::new();
    for d in seq {
        s.push_str(&format!("{}", d));
    }
    s
}

pub const NUMPAD_POS_START: PosNum = PosNum::new_static::<2, 3>();

pub fn numpad_use(buttons: &[KeyCell]) -> Vec<NumCell> {
    let g = numpad_get();
    let mut pos = NUMPAD_POS_START;
    let mut r = vec![];
    for button in buttons {
        match button {
            KeyCell::Invalid => panic!("went over invalid!"),
            KeyCell::D(dir) => pos = (pos + *dir).expect("invalid direction"),
            KeyCell::A => r.push(g[pos]),
        }
    }
    r
}

pub fn numpad_dfs(
    src: PosNum,
    pos0: PosNum,
    path: &mut Vec<KeyCell>,
    bestpaths: &mut HashMap<(NumCell, NumCell), Vec<Vec<KeyCell>>>,
) {
    let g = numpad_get();
    for dir in Dir::iter::<false>() {
        let Some(pos) = (pos0 + dir)
            .ok()
            .filter(|p| *p != src && g[p] != NumCell::Invalid)
        else {
            continue;
        };
        path.push(KeyCell::D(dir));
        let e = bestpaths.entry((g[src], g[pos])).or_default();
        if e.is_empty() || path.len() <= e[0].len() {
            path.push(KeyCell::A);
            if e.is_empty() || path.len() < e[0].len() {
                *e = vec![path.clone()];
            } else if path.len() == e[0].len() {
                e.push(path.clone());
            }
            path.pop();
            numpad_dfs(src, pos, path, bestpaths);
        }
        path.pop();
    }
}

pub fn numpad_bestpaths_all() -> HashMap<(NumCell, NumCell), Vec<Vec<KeyCell>>> {
    let g = numpad_get();
    let mut bestpaths = HashMap::<(NumCell, NumCell), Vec<Vec<KeyCell>>>::new();
    for p in PosNum::iter() {
        if g[p] == NumCell::Invalid {
            continue;
        }
        bestpaths.insert((g[p], g[p]), vec![]);
        let mut path = vec![];
        numpad_dfs(p, p, &mut path, &mut bestpaths);
    }
    bestpaths
}

pub fn numpad_sequences(code: &[NumCell]) -> Vec<Vec<KeyCell>> {
    let bestpaths = numpad_bestpaths_all();
    let mut prev = NumCell::A;
    let mut paths: Vec<Vec<KeyCell>> = vec![];
    for target in code {
        let mut newpaths = vec![];
        if paths.is_empty() {
            for newsegment in &bestpaths[&(prev, *target)] {
                newpaths.push(newsegment.clone());
            }
        } else {
            for path in &paths {
                for newsegment in &bestpaths[&(prev, *target)] {
                    let mut newpath = path.clone();
                    newpath.extend(newsegment.iter());
                    newpaths.push(newpath);
                }
            }
        }
        prev = *target;
        let _ = std::mem::replace(&mut paths, newpaths);
    }
    paths
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
