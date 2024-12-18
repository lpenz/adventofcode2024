// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use day17::*;

pub const EXAMPLE_FIXPOINT: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn distance(cpu: &Computer) -> usize {
    (0..(std::cmp::max(cpu.prog_vec.len(), cpu.output.len())))
        .map(|i| {
            if i > cpu.output.len() || cpu.output.len() != cpu.prog_vec.len() {
                128
            } else {
                let o = cpu.output[i] as usize;
                let p = cpu.prog_vec[i] as usize;
                if o > p {
                    o - p
                } else {
                    p - o
                }
            }
        })
        .sum()
}

pub type Node = [u8; 16];

fn node_to_a(n: Node) -> u64 {
    n.into_iter()
        .enumerate()
        .map(|(i, v)| {
            let v = v as u64;
            v << ((n.len() - i - 1) * 3)
        })
        .sum()
}

fn pushit(frontier: &mut BinaryHeap<(Reverse<usize>, Node)>, cpu: &Computer, n: Node) {
    let a = node_to_a(n);
    let mut c = cpu.clone();
    c.regs[Reg::A] = a;
    c.run();
    let dist = distance(&c);
    frontier.push((Reverse(dist), n));
}

fn find_fixpoint(cpu: Computer) -> Num {
    let mut frontier = BinaryHeap::<(Reverse<usize>, Node)>::new();
    pushit(&mut frontier, &cpu, Node::default());
    let mut visited = HashSet::<Node>::new();
    let mut mindist = usize::MAX;
    while let Some((Reverse(dist), node)) = frontier.pop() {
        if dist == 0 {
            return node_to_a(node);
        }
        if dist < mindist {
            mindist = dist;
            eprintln!("mindist {}", mindist);
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for i in 0..node.len() {
            if node[i] > 0 {
                let mut n = node;
                n[i] -= 1;
                pushit(&mut frontier, &cpu, n);
            }
            if node[i] < 7 {
                let mut n = node;
                n[i] += 1;
                pushit(&mut frontier, &cpu, n);
            }
        }
    }
    panic!("could not find answer")
}

fn process(bufin: impl BufRead) -> Result<Num> {
    let cpu = parser::parse(bufin)?;
    Ok(find_fixpoint(cpu))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE_FIXPOINT.as_bytes())?, 117440);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
