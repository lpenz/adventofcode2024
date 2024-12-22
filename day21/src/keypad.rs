// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use sqrid::Dir;
use sqrid::PosT;

use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeyCell {
    #[default]
    Invalid,
    D(Dir),
    A,
}

impl From<char> for KeyCell {
    fn from(c: char) -> Self {
        match c {
            '^' => KeyCell::D(Dir::N),
            '>' => KeyCell::D(Dir::E),
            'v' => KeyCell::D(Dir::S),
            '<' => KeyCell::D(Dir::W),
            'A' => KeyCell::A,
            _ => panic!("unknown cell {}", c),
        }
    }
}

impl std::fmt::Display for KeyCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyCell::A => 'A',
                KeyCell::D(Dir::N) => '^',
                KeyCell::D(Dir::E) => '>',
                KeyCell::D(Dir::S) => 'v',
                KeyCell::D(Dir::W) => '<',
                _ => panic!("can't express invalid keycell"),
            }
        )
    }
}

pub type SqridKey = sqrid::sqrid_create!(2, 1, false);
pub type PosKey = sqrid::pos_create!(SqridKey);
pub type GridKey = sqrid::grid_create!(SqridKey, KeyCell);

pub fn keypad_get() -> GridKey {
    [
        KeyCell::Invalid,
        KeyCell::D(Dir::N),
        KeyCell::A,
        KeyCell::D(Dir::W),
        KeyCell::D(Dir::S),
        KeyCell::D(Dir::E),
    ]
    .into_iter()
    .collect::<GridKey>()
}

pub fn keyseq2str(seq: &[KeyCell]) -> String {
    let mut s = String::new();
    for d in seq {
        s.push_str(&format!("{}", d));
    }
    s
}

pub const KEYPAD_POS_START: PosKey = PosKey::new_static::<2, 0>();

pub fn keypad_use(buttons: &[KeyCell]) -> Vec<KeyCell> {
    let g = keypad_get();
    let mut pos = KEYPAD_POS_START;
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

pub fn keypad_dfs(
    src: PosKey,
    pos0: PosKey,
    path: &mut Vec<KeyCell>,
    bestpaths: &mut HashMap<(KeyCell, KeyCell), Vec<Vec<KeyCell>>>,
) {
    let g = keypad_get();
    for dir in Dir::iter::<false>() {
        let Some(pos) = (pos0 + dir)
            .ok()
            .filter(|p| *p != src && g[p] != KeyCell::Invalid)
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
            keypad_dfs(src, pos, path, bestpaths);
        }
        path.pop();
    }
}

pub fn keypad_bestpaths_all() -> HashMap<(KeyCell, KeyCell), Vec<Vec<KeyCell>>> {
    let g = keypad_get();
    let mut bestpaths = HashMap::<(KeyCell, KeyCell), Vec<Vec<KeyCell>>>::new();
    for p in PosKey::iter() {
        if g[p] == KeyCell::Invalid {
            continue;
        }
        bestpaths.insert((g[p], g[p]), vec![vec![KeyCell::A]]);
        let mut path = vec![];
        keypad_dfs(p, p, &mut path, &mut bestpaths);
    }
    bestpaths
}

pub fn keypad_sequences(code: &[KeyCell]) -> Vec<Vec<KeyCell>> {
    let bestpaths = keypad_bestpaths_all();
    let mut prev = KeyCell::A;
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
