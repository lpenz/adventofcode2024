// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day24::*;

use std::collections::HashMap;
use std::collections::HashSet;

fn swap(conns: &mut HashMap<Wire, Gate>, w1: Wire, w2: Wire) {
    let e1 = conns.get_mut(&w1).unwrap() as *mut Gate;
    let e2 = conns.get_mut(&w2).unwrap() as *mut Gate;
    unsafe {
        std::ptr::swap(e1, e2);
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (init, connections) = parser::parse(bufin)?;
    println!("digraph {{");
    println!("  rankdir=LR");
    let inputs = init.iter().map(|(w, _)| w).collect::<HashSet<_>>();
    for i in 0..usize::MAX {
        let wire = Wire(format!("x{:02}", i).as_str().try_into().unwrap());
        if !inputs.contains(&wire) {
            break;
        }
        println!("  subgraph cluster_{} {{", i);
        println!("    x{:02}", i);
        println!("    y{:02}", i);
        println!("  }}");
        if i > 0 {
            println!("  y{:02} -> x{:02} [ style=invis ]", i - 1, i);
        }
    }
    let mut connections = connections.into_iter().collect::<HashMap<Wire, Gate>>();
    swap(&mut connections, Wire::new("z06"), Wire::new("fhc"));
    swap(&mut connections, Wire::new("z35"), Wire::new("hqk"));
    swap(&mut connections, Wire::new("ggt"), Wire::new("mwh"));
    swap(&mut connections, Wire::new("z11"), Wire::new("qhj"));
    for (i, (out, gate)) in connections.into_iter().enumerate() {
        println!("  op{} [ label=\"{:?}\" ]", i, gate.op);
        for input in &gate.inputs {
            println!("  {} -> op{}", input.0, i);
        }
        println!("  op{} -> {}", i, out.0);
    }
    println!("}}");
    Ok(0)
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
