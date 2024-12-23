// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use sqrid::Dir;
use sqrid::PosT;

use std::sync::OnceLock;

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

pub const KEYPAD_POS_START: PosKey = PosKey::new_static::<2, 0>();

#[derive(Clone)]
pub struct Keypad {
    pub grid: GridKey,
    pub paths: HashMap<(KeyCell, KeyCell), Vec<Vec<KeyCell>>>,
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}

impl Keypad {
    pub fn new() -> Self {
        let grid = [
            KeyCell::Invalid,
            KeyCell::D(Dir::N),
            KeyCell::A,
            KeyCell::D(Dir::W),
            KeyCell::D(Dir::S),
            KeyCell::D(Dir::E),
        ]
        .into_iter()
        .collect::<GridKey>();
        Self {
            grid,
            paths: keypad_bestpaths(&grid),
        }
    }

    pub fn press(&self, buttons: &[KeyCell]) -> Vec<KeyCell> {
        buttons
            .iter()
            .fold((KEYPAD_POS_START, vec![]), |(mut pos, mut ret), button| {
                match button {
                    KeyCell::Invalid => panic!("went over invalid!"),
                    KeyCell::D(dir) => pos = (pos + *dir).expect("invalid direction"),
                    KeyCell::A => ret.push(self.grid[pos]),
                }
                (pos, ret)
            })
            .1
    }

    pub fn paths_get(&self, start: &KeyCell, end: &KeyCell) -> impl Iterator<Item = &Vec<KeyCell>> {
        self.paths.get(&(*start, *end)).unwrap().iter()
    }
}

pub fn keyseq2str(seq: &[KeyCell]) -> String {
    let mut s = String::new();
    for d in seq {
        s.push_str(&format!("{}", d));
    }
    s
}

fn keypad_dfs(
    g: &GridKey,
    src: PosKey,
    pos0: PosKey,
    path: &mut Vec<KeyCell>,
    dpath: &mut Vec<Dir>,
    bestpaths: &mut HashMap<(KeyCell, KeyCell), Vec<Vec<KeyCell>>>,
) {
    for dir in Dir::iter::<false>() {
        if dpath.contains(&dir) && dpath[dpath.len() - 1] != dir {
            continue;
        }
        let Some(pos) = (pos0 + dir)
            .ok()
            .filter(|p| *p != src && g[p] != KeyCell::Invalid)
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
            keypad_dfs(g, src, pos, path, dpath, bestpaths);
        }
        path.pop();
        dpath.pop();
    }
}

fn keypad_bestpaths(g: &GridKey) -> HashMap<(KeyCell, KeyCell), Vec<Vec<KeyCell>>> {
    let mut bestpaths = HashMap::<(KeyCell, KeyCell), Vec<Vec<KeyCell>>>::new();
    for p in PosKey::iter() {
        if g[p] == KeyCell::Invalid {
            continue;
        }
        bestpaths.insert((g[p], g[p]), vec![vec![KeyCell::A]]);
        let mut path = vec![];
        let mut dpath = vec![];
        keypad_dfs(g, p, p, &mut path, &mut dpath, &mut bestpaths);
    }
    bestpaths
}

pub fn keypad_get() -> &'static Keypad {
    static MEM: OnceLock<Keypad> = OnceLock::new();
    MEM.get_or_init(Keypad::default)
}

#[test]
fn test_keypad() {
    let keypad = keypad_get();
    assert_eq!(keypad.paths[&(KeyCell::A, KeyCell::D(Dir::W))].len(), 1);
    assert_eq!(keypad.paths[&(KeyCell::D(Dir::W), KeyCell::A)].len(), 1);
}
