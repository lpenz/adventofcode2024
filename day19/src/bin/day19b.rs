// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day19::*;

use cached::proc_macro::cached;
use cached::SizedCache;
use rayon::prelude::*;

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(200) }",
    convert = r#"{ format!("{:?}~{:?}", towels, design) }"#
)]
fn ways(towels: &[Towel], design: &[Color]) -> usize {
    if design.is_empty() {
        return 1;
    }
    towels.iter().fold(0, |mut count, t| {
        let tl = t.len();
        if tl <= design.len() && &design[0..tl] == t.as_slice() {
            count += ways(towels, &design[tl..]);
        }
        count
    })
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (towels, designs) = parser::parse(bufin)?;
    Ok(designs
        .into_par_iter()
        .map(|design| ways(&towels, &design))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 16);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
