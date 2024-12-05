// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day05::*;

use std::collections::HashMap;
use std::collections::HashSet;

type Rules = HashSet<(u32, u32)>;

fn valid(rules: &Rules, update: &[u32]) -> bool {
    let pages = update
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<u32, usize>>();
    rules.iter().all(|rule| {
        let Some(page1) = pages.get(&rule.0) else {
            return true;
        };
        let Some(page2) = pages.get(&rule.1) else {
            return true;
        };
        page1 < page2
    })
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let (rules0, updates) = parser::parse(bufin)?;
    let rules = rules0.into_iter().collect::<Rules>();
    Ok(updates
        .into_iter()
        .filter(|upd| valid(&rules, upd))
        .map(|upd| upd[upd.len() / 2])
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 143);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
