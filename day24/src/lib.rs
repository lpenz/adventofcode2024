// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub mod parser;
pub use parser::EXAMPLE1;
pub use parser::EXAMPLE2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Wire(pub copstr::Str<3>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn apply(&self, input0: bool, input1: bool) -> bool {
        match self {
            Self::And => input0 && input1,
            Self::Or => input0 || input1,
            Self::Xor => input0 ^ input1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gate {
    pub inputs: [Wire; 2],
    pub op: Operation,
}

impl Gate {
    pub fn new(wire1: Wire, wire2: Wire, op: Operation) -> Self {
        Self {
            inputs: [wire1, wire2],
            op,
        }
    }
}

pub type Connection = (Wire, Gate);
