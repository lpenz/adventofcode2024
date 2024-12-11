// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let nfiles = (input.len() + 1) / 2;
    let mut fblocks = (0..nfiles)
        .map(|id| (id, input[id * 2] as usize))
        .collect::<HashMap<usize, usize>>();
    let mut r = 0_usize;
    let mut id = 0;
    let mut block = 0;
    'outer: for (i, nblocks) in input.into_iter().enumerate() {
        for b in block..(block + nblocks) {
            if i % 2 == 0 {
                // File
                if let Some(e) = fblocks.get_mut(&id) {
                    r += b * id;
                    *e -= 1;
                    if *e == 0 && b < block + nblocks - 1 {
                        // No more blocks for this file, we are done
                        break 'outer;
                    }
                }
            } else {
                // Empty, bring from last id
                let id = *fblocks.keys().max().unwrap();
                let mut remove = false;
                if let Some(e) = fblocks.get_mut(&id) {
                    *e -= 1;
                    if *e == 0 {
                        remove = true;
                    }
                }
                if remove {
                    fblocks.remove(&id);
                }
                r += b * id;
            }
        }
        block += nblocks;
        if i % 2 == 0 {
            id += 1;
        }
    }
    Ok(r)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1928);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
