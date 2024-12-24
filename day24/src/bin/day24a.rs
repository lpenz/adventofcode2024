// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day24::*;

use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Solver {
    pub values: HashMap<Wire, bool>,
    pub formulas: HashMap<Wire, Gate>,
}

impl Solver {
    pub fn new(init: Vec<(Wire, bool)>, conns: Vec<Connection>) -> Self {
        let mut solver = Solver::default();
        for (wire, value) in init {
            solver.values.insert(wire, value);
        }
        for conn in conns {
            solver.formulas.insert(conn.0, conn.1);
        }
        solver
    }

    pub fn solve(&mut self, wire: Wire) -> bool {
        if let Some(v) = self.values.get(&wire) {
            return *v;
        }
        let Some(&formula) = self.formulas.get(&wire) else {
            panic!("no formula found for {:?}", wire);
        };
        let input0 = self.solve(formula.inputs[0]);
        let input1 = self.solve(formula.inputs[1]);
        let output = formula.op.apply(input0, input1);
        self.values.insert(wire, output);
        output
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (initial, connections) = parser::parse(bufin)?;
    let zoutputs = connections
        .iter()
        .filter_map(|conn| {
            if conn.0 .0.starts_with('z') {
                Some(Reverse(conn.0))
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let mut solver = Solver::new(initial, connections);
    Ok(zoutputs.into_iter().fold(0, |mut acc, Reverse(wire)| {
        acc <<= 1;
        acc |= solver.solve(wire) as usize;
        acc
    }))
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 4);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 2024);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
