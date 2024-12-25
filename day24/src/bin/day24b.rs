// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day24::*;

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub type Swapper = HashMap<Wire, Wire>;

pub fn swapper_from(swaps: &[(Wire, Wire)]) -> Swapper {
    swaps
        .iter()
        .copied()
        .flat_map(|(w1, w2)| [(w1, w2), (w2, w1)])
        .collect()
}

#[derive(Debug, Default, Clone)]
pub struct Circuit {
    pub formulas: HashMap<Wire, Gate>,
    pub in_size: usize,
    pub out_size: usize,
}

impl Circuit {
    pub fn new(formulas: HashMap<Wire, Gate>) -> Self {
        let in_size = formulas
            .values()
            .flat_map(|g| g.inputs.iter())
            .filter_map(|wire| wire.index())
            .max()
            .unwrap()
            + 1;
        let out_size = (0..usize::MAX)
            .find(|&i| !formulas.contains_key(&Wire::new_z(i)))
            .unwrap();
        Self {
            formulas,
            in_size,
            out_size,
        }
    }

    pub fn swaps(&self) -> Vec<(Wire, Wire)> {
        self.formulas
            .keys()
            .copied()
            .sorted()
            .tuple_combinations()
            .collect::<Vec<_>>()
    }

    pub fn solve_dfs(
        &self,
        values: &mut HashMap<Wire, bool>,
        swapper: &Swapper,
        mut wire: Wire,
        stack: &mut Vec<Wire>,
    ) -> Result<bool> {
        if let Some(w) = swapper.get(&wire) {
            wire = *w;
        }
        if let Some(v) = values.get(&wire) {
            return Ok(*v);
        }
        let Some(&formula) = self.formulas.get(&wire) else {
            panic!("no formula found for {:?}", wire);
        };
        if formula.inputs.iter().any(|w| stack.contains(w)) {
            // cycle
            return Err(eyre!("loop detected"));
        }
        let inputs = formula
            .inputs
            .iter()
            .map(|&inputwire| {
                stack.push(inputwire);
                let value = self.solve_dfs(values, swapper, inputwire, stack)?;
                stack.pop();
                Ok(value)
            })
            .collect::<Result<Vec<bool>, Report>>()?;
        let output = formula.op.apply(inputs[0], inputs[1]);
        values.insert(wire, output);
        Ok(output)
    }

    pub fn solve(
        &self,
        values: &mut HashMap<Wire, bool>,
        swapper: &Swapper,
        wire: Wire,
    ) -> Result<bool> {
        let mut stack = vec![];
        self.solve_dfs(values, swapper, wire, &mut stack)
    }
}

pub trait Solver {
    fn set_circuit(&mut self, circuit: Circuit);
    fn get_circuit(&self) -> &Circuit;

    fn use_circuit(&self, swapper: &Swapper, x: u64, y: u64) -> Result<u64> {
        let mut values = HashMap::<Wire, bool>::new();
        for i in 0..self.get_circuit().in_size {
            let mask = 1 << i;
            let xwire = Wire::new_x(i);
            values.insert(xwire, x & mask > 0);
            let ywire = Wire::new_y(i);
            values.insert(ywire, y & mask > 0);
        }
        let mut result = 0_u64;
        for i in 0..self.get_circuit().out_size {
            let zwire = Wire::new_z(i);
            if self.get_circuit().solve(&mut values, swapper, zwire)? {
                result |= 1 << i;
            }
        }
        Ok(result)
    }

    fn bit_ok(&self, swapper: &Swapper, i: usize) -> Result<bool> {
        let mask = 1_u64 << i;
        let result = self.use_circuit(swapper, mask, 0)?;
        if result != self.use_maths(mask, 0) {
            return Ok(false);
        }
        let result = self.use_circuit(swapper, mask, mask)?;
        if result != self.use_maths(mask, mask) {
            return Ok(false);
        }
        Ok(true)
    }

    fn error_mask(&self, swapper: &Swapper) -> Result<u64> {
        let mut acc = 0;
        for i in 0..self.get_circuit().in_size {
            let ok = self.bit_ok(swapper, i)?;
            if !ok {
                acc |= 1_u64 << i;
            }
        }
        Ok(acc)
    }

    fn test(&self, swapper: &Swapper, n1: u64, n2: u64) -> bool {
        let Ok(answer) = self.use_circuit(swapper, n1, n2) else {
            return false;
        };
        if answer != self.use_maths(n1, n2) {
            return false;
        }
        true
    }

    fn fulltest(&self, swapper: &Swapper) -> bool {
        let mask = (0..self.get_circuit().in_size)
            .filter_map(|b| (b % 2 == 0).then_some(1 << b))
            .sum();
        if !self.test(swapper, mask, mask) {
            return false;
        }
        let mask = (0..self.get_circuit().in_size)
            .filter_map(|b| (b % 2 != 0).then_some(1 << b))
            .sum();
        if !self.test(swapper, mask, mask) {
            return false;
        }
        for i in 1..self.get_circuit().in_size {
            let mask = (1 << i) - 1;
            if !self.test(swapper, mask, 1) {
                return false;
            }
        }
        true
    }

    fn use_maths(&self, x: u64, y: u64) -> u64;
}

#[derive(Debug, Default)]
pub struct SolveAdder {
    pub circuit: Circuit,
}

impl Solver for SolveAdder {
    fn set_circuit(&mut self, circuit: Circuit) {
        self.circuit = circuit;
    }

    fn get_circuit(&self) -> &Circuit {
        &self.circuit
    }

    fn use_maths(&self, x: u64, y: u64) -> u64 {
        x + y
    }
}

#[derive(Debug, Default)]
pub struct SolveAnd {
    pub circuit: Circuit,
}

impl Solver for SolveAnd {
    fn set_circuit(&mut self, circuit: Circuit) {
        self.circuit = circuit;
    }

    fn get_circuit(&self) -> &Circuit {
        &self.circuit
    }

    fn use_maths(&self, x: u64, y: u64) -> u64 {
        x & y
    }
}

fn check_candidate<S: Solver>(
    solver: &S,
    stack: &mut Vec<(Wire, Wire)>,
    bit_swap_candidates: &[Vec<(Wire, Wire)>],
) -> Option<Vec<(Wire, Wire)>> {
    if bit_swap_candidates.is_empty() {
        let swapper = swapper_from(stack);
        if solver.fulltest(&swapper) {
            return Some(stack.clone());
        }
    } else {
        for swap in &bit_swap_candidates[0] {
            stack.push(*swap);
            if let Some(solution) = check_candidate(solver, stack, &bit_swap_candidates[1..]) {
                return Some(solution);
            }
            stack.pop();
        }
    }
    None
}

fn process<S: Solver + Sync>(mut solver: S, bufin: impl BufRead) -> Result<String> {
    let (_, connections) = parser::parse(bufin)?;
    let formulas = connections.into_iter().collect::<HashMap<Wire, Gate>>();
    let circuit = Circuit::new(formulas);
    solver.set_circuit(circuit);
    let swaps = solver.get_circuit().swaps();
    // Collect candidates for each bit that fails:
    let bit_swap_candidates: Vec<Vec<(Wire, Wire)>> = (0..solver.get_circuit().in_size - 1)
        .into_par_iter()
        .filter(|i| !solver.bit_ok(&Swapper::default(), *i).is_ok_and(|v| v))
        .map(|i| {
            let solver = &solver;
            swaps
                .par_iter()
                .filter_map(move |swap| {
                    let swapper = swapper_from(&[*swap]);
                    if !solver.bit_ok(&swapper, i).is_ok_and(|v| v) {
                        return None;
                    }
                    Some(*swap)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // Check the candidates together using solver.fulltest
    let mut stack = vec![];
    let solution =
        check_candidate(&solver, &mut stack, &bit_swap_candidates).expect("no solution found");
    // Build the output in the desired format
    Ok(solution
        .into_iter()
        .flat_map(|(Wire(w1), Wire(w2))| [format!("{}", w1), format!("{}", w2)])
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, s)| {
            // It's a shame that intersperse is not yet stable
            if i > 0 {
                format!(",{}", s)
            } else {
                s
            }
        })
        .collect::<String>())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        process(SolveAnd::default(), EXAMPLE3.as_bytes())?,
        "z00,z01,z02,z05"
    );
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(SolveAdder::default(), stdin().lock()))
}
