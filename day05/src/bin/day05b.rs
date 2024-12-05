// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day05::*;

use std::collections::HashMap;
use std::collections::HashSet;

type Rules = HashSet<(u32, u32)>;

fn fix(rules0: &Rules, update: Vec<u32>) -> Option<Vec<u32>> {
    let used = update.iter().copied().collect::<HashSet<_>>();
    let filtered_rules = rules0
        .iter()
        .filter(|r| used.contains(&r.0) && used.contains(&r.1))
        .copied()
        .collect::<Rules>();
    let mut rules = HashMap::<u32, usize>::default();
    for rule in filtered_rules {
        rules.entry(rule.0).or_default();
        *rules.entry(rule.1).or_default() += 1;
    }
    let mut fixed = rules
        .into_iter()
        .map(|(p, num)| (num, p))
        .collect::<Vec<_>>();
    fixed.sort();
    let fixed = fixed.into_iter().map(|(_, p)| p).collect::<Vec<_>>();
    if fixed != update {
        Some(fixed)
    } else {
        None
    }
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let (rules0, updates) = parser::parse(bufin)?;
    let rules = rules0.into_iter().collect::<Rules>();
    Ok(updates
        .into_iter()
        .filter_map(|upd| fix(&rules, upd))
        .map(|upd| upd[upd.len() / 2])
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 123);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
