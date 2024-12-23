// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day23::*;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solver {
    conn_set: HashSet<Connection>,
    cpus: BTreeSet<Cpu>,
    conn_map: HashMap<Cpu, BTreeSet<Cpu>>,
}

impl Solver {
    pub fn new(conn_vec: Vec<Connection>) -> Self {
        let conn_set = conn_vec.into_iter().collect::<HashSet<_>>();
        let cpus = conn_set
            .iter()
            .flat_map(|conn| [conn.0, conn.1].into_iter())
            .collect::<BTreeSet<_>>();
        let mut conn_map = HashMap::<Cpu, BTreeSet<Cpu>>::default();
        for &cpu1 in &cpus {
            for &cpu2 in cpus.iter().filter(|c| *c > &cpu1) {
                if !conn_set.contains(&(cpu1, cpu2)) {
                    continue;
                }
                let e = conn_map.entry(cpu1).or_default();
                e.insert(cpu2);
            }
        }
        Solver {
            conn_set,
            cpus,
            conn_map,
        }
    }

    pub fn check_fully_connected(&self, cpus: &BTreeSet<Cpu>) -> bool {
        for &cpu1 in cpus {
            for &cpu2 in cpus.iter().filter(|c| *c > &cpu1) {
                if !self.conn_set.contains(&(cpu1, cpu2)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn max_clique_dfs(&self, clique: &mut BTreeSet<Cpu>) -> BTreeSet<Cpu> {
        let cpu0 = clique.last().unwrap();
        if !self.conn_map.contains_key(cpu0) {
            // Max cpu, nothing to check
            return clique.clone();
        }
        let mut best = clique.clone();
        for cpu in &self.conn_map[cpu0] {
            clique.insert(*cpu);
            if self.check_fully_connected(clique) {
                let next = self.max_clique_dfs(clique);
                if next.len() > best.len() {
                    best = next.clone();
                }
            }
            clique.remove(cpu);
        }
        best
    }

    pub fn max_clique(&self) -> BTreeSet<Cpu> {
        let mut best = BTreeSet::<Cpu>::default();
        for &cpu in &self.cpus {
            let mut clique = BTreeSet::<Cpu>::default();
            clique.insert(cpu);
            let next = self.max_clique_dfs(&mut clique);
            if next.len() > best.len() {
                best = next.clone();
            }
            clique.remove(&cpu);
        }
        best
    }
}

pub fn cpus_to_str(cpus: &BTreeSet<Cpu>) -> String {
    let mut s = String::default();
    for (i, cpu) in cpus.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(format!("{}", cpu.0).as_str());
    }
    s
}

fn process(bufin: impl BufRead) -> Result<String> {
    let conn_set = parser::parse(bufin)?;
    let solver = Solver::new(conn_set);
    let best = solver.max_clique();
    Ok(cpus_to_str(&best))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, "co,de,ka,ta".to_string());
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
