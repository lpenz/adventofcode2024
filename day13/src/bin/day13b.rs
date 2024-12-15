// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day13::*;

use std::ops::Add;
use std::ops::Mul;
use z3::ast::Ast;
use z3::{Config, Context, Solver};

fn calc(a: XY, b: XY, prize: XY) -> Num {
    let ctx = &Context::new(&Config::default());
    let solver = Solver::new(ctx);
    // Variables to solve:
    let apress = &z3::ast::Int::new_const(ctx, "a");
    let bpress = &z3::ast::Int::new_const(ctx, "b");
    // Constants:
    let ax = &z3::ast::Int::from_u64(ctx, a.re);
    let ay = &z3::ast::Int::from_u64(ctx, a.im);
    let bx = &z3::ast::Int::from_u64(ctx, b.re);
    let by = &z3::ast::Int::from_u64(ctx, b.im);
    let px = &z3::ast::Int::from_u64(ctx, prize.re);
    let py = &z3::ast::Int::from_u64(ctx, prize.im);
    // Equations:
    solver.assert(&px._eq(&apress.mul(ax).add(bpress.mul(bx))));
    solver.assert(&py._eq(&apress.mul(ay).add(bpress.mul(by))));
    // Let's solve it:
    if solver.check() != z3::SatResult::Sat {
        return 0;
    }
    // Get the solution:
    let model = solver.get_model().unwrap();
    let apress = model.eval(apress, true).and_then(|v| v.as_u64()).unwrap();
    let bpress = model.eval(bpress, true).and_then(|v| v.as_u64()).unwrap();
    apress * 3 + bpress
}

fn process(add: Num, bufin: impl BufRead) -> Result<Num> {
    let machines = parser::parse(bufin)?;
    Ok(machines
        .into_iter()
        .map(|(a, b, prize)| calc(a, b, Complex::new(add, add) + prize))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(0, EXAMPLE.as_bytes())?, 480);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let m = parser::parse(EXAMPLE.as_bytes())?;
    let add = Complex::new(10000000000000, 10000000000000);
    let m = m
        .into_iter()
        .map(|(a, b, prize)| (a, b, add + prize))
        .collect::<Vec<_>>();
    assert!(calc(m[0].0, m[0].1, m[0].2) == 0);
    assert!(calc(m[1].0, m[1].1, m[1].2) > 0);
    assert!(calc(m[2].0, m[2].1, m[2].2) == 0);
    assert!(calc(m[3].0, m[3].1, m[3].2) > 0);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(10000000000000, stdin().lock()))
}
