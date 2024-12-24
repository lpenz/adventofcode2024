// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use aoc::parser::*;

pub const EXAMPLE1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

pub const EXAMPLE2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

use super::*;

type ParseResult = (Vec<(Wire, bool)>, Vec<Connection>);

fn wire(input: &str) -> IResult<&str, Wire> {
    let (input, name) = character::alphanumeric1(input)?;
    Ok((input, Wire(name.try_into().unwrap())))
}

fn wire_init(input: &str) -> IResult<&str, (Wire, bool)> {
    let (input, wire) = wire(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = character::one_of("01")(input)?;
    let (input, _) = character::newline(input)?;
    Ok((input, (wire, value == '1')))
}

fn operation_and(input: &str) -> IResult<&str, Operation> {
    tag("AND")(input).map(|(input, _)| (input, Operation::And))
}

fn operation_or(input: &str) -> IResult<&str, Operation> {
    tag("OR")(input).map(|(input, _)| (input, Operation::Or))
}

fn operation_xor(input: &str) -> IResult<&str, Operation> {
    tag("XOR")(input).map(|(input, _)| (input, Operation::Xor))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) =
        branch::alt((operation_and, branch::alt((operation_or, operation_xor))))(input)?;
    Ok((input, operation))
}

fn connection(input: &str) -> IResult<&str, Connection> {
    let (input, wire1) = wire(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, wire2) = wire(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, output) = wire(input)?;
    let (input, _) = character::newline(input)?;
    Ok((input, (output, Gate::new(wire1, wire2, op))))
}

fn all(input: &str) -> IResult<&str, ParseResult> {
    let (input, inits) = multi::many1(wire_init)(input)?;
    let (input, _) = character::newline(input)?;
    let (input, conns) = multi::many1(connection)(input)?;
    Ok((input, (inits, conns)))
}

pub fn parse(mut bufin: impl BufRead) -> Result<ParseResult> {
    aoc::parse_with!(all, bufin)
}

#[test]
fn test1() -> Result<()> {
    let input = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(input.0.len(), 6);
    assert_eq!(input.1.len(), 3);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input = parser::parse(EXAMPLE2.as_bytes())?;
    assert_eq!(input.0.len(), 10);
    assert_eq!(input.1.len(), 36);
    Ok(())
}
