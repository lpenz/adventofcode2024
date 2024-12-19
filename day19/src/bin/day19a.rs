// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day19::*;

use cached::proc_macro::cached;
use cached::SizedCache;
use rayon::prelude::*;

#[cached(
    ty = "SizedCache<String, bool>",
    create = "{ SizedCache::with_size(200) }",
    convert = r#"{ format!("{:?}~{:?}", towels, design) }"#
)]
fn possible(towels: &[Towel], design: &[Color]) -> bool {
    if design.is_empty() {
        return true;
    }
    for t in towels {
        let tl = t.len();
        if tl <= design.len() && &design[0..tl] == t.as_slice() && possible(towels, &design[tl..]) {
            return true;
        }
    }
    false
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (towels, designs) = parser::parse(bufin)?;
    Ok(designs
        .into_par_iter()
        .filter(|design| possible(&towels, design))
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 6);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
