// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::BTreeMap;

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let nfiles = (input.len() + 1) / 2;
    let mut fblocks = (0..nfiles)
        .map(|id| (id, input[id * 2] as usize))
        .collect::<BTreeMap<usize, usize>>();
    let mut r = 0_usize;
    let mut id = 0;
    let mut block = 0;
    'outer: for (i, mut nblocks) in input.into_iter().enumerate() {
        if i % 2 == 0 {
            // File
            for b in block..(block + nblocks) {
                if let Some(e) = fblocks.get_mut(&id) {
                    r += b * id;
                    *e -= 1;
                    if *e == 0 && b < block + nblocks - 1 {
                        // No more blocks for this file, we are done
                        break 'outer;
                    }
                }
            }
            block += nblocks;
            id += 1;
        } else {
            // Empty, bring from last id only if it fits
            while nblocks > 0 {
                if let Some((&id, &size)) = fblocks
                    .iter()
                    .rev()
                    .filter_map(|(id, size)| (*size <= nblocks).then_some((id, size)))
                    .next()
                {
                    fblocks.remove(&id);
                    for b in block..(block + size) {
                        r += b * id;
                    }
                    block += size;
                    nblocks -= size;
                } else {
                    block += nblocks;
                    break;
                }
            }
        }
    }
    Ok(r)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2858);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
