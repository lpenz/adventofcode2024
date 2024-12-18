// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;

use day17::*;

pub const EXAMPLE_FIXPOINT: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn test_fixpoint(cpu: &Computer, a: Num) -> bool {
    let mut cpu = cpu.clone();
    cpu.regs[Reg::A] = a;
    // cpu.run();
    while !cpu.halted() {
        cpu.once();
        let l = cpu.output.len();
        if l > cpu.prog_vec.len() || cpu.output != cpu.prog_vec[0..l] {
            // eprintln!("{} # {:?} != {:?}", a, cpu.output, cpu.prog_vec);
            return false;
        }
    }
    // eprintln!("{} # {:?} != {:?} end", a, cpu.output, cpu.prog_vec);
    cpu.output == cpu.prog_vec
}

fn find_fixpoint(cpu: Computer) -> Num {
    (0..Num::MAX)
        // (0..500000)
        .into_par_iter()
        .find_any(|a| test_fixpoint(&cpu, *a))
        .expect("could not find a fixpoint")
}

fn process(bufin: impl BufRead) -> Result<Num> {
    let cpu = parser::parse(bufin)?;
    Ok(find_fixpoint(cpu))
}

#[test]
fn test() {
    let cpu = parser::parse(EXAMPLE_FIXPOINT.as_bytes()).unwrap();
    assert!(test_fixpoint(&cpu, 117440));
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE_FIXPOINT.as_bytes())?, 117440);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
