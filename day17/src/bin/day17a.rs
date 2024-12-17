// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day17::*;

fn process(bufin: impl BufRead) -> Result<String> {
    let mut cpu = parser::parse(bufin)?;
    cpu.run();
    Ok(cpu.output_str())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, "4,6,3,5,6,3,5,2,1,0");
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
