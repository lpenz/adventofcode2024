// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

pub struct Lock(pub [u8; 5]);
pub struct Key(pub [u8; 5]);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    enum LockKey {
        Lock(Lock),
        Key(Key),
    }

    fn cell(input: &str) -> IResult<&str, char> {
        let (input, g) = character::one_of(".#")(input)?;
        Ok((input, g))
    }

    fn lockkey(input: &str) -> IResult<&str, [u8; 5]> {
        let (input, _) = character::newline(input)?;
        let (input, g) = grid(cell)(input)?;
        let contents = (0_usize..5)
            .map(|x| (0_usize..6).filter(|y| g[*y][x] == '#').count() as u8)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok((input, contents))
    }

    fn key(input: &str) -> IResult<&str, LockKey> {
        let (input, _) = tag(".....")(input)?;
        let (input, c) = lockkey(input)?;
        Ok((input, LockKey::Key(Key(c))))
    }

    fn lock(input: &str) -> IResult<&str, LockKey> {
        let (input, _) = tag("#####")(input)?;
        let (input, c) = lockkey(input)?;
        Ok((input, LockKey::Lock(Lock(c))))
    }

    fn all(input: &str) -> IResult<&str, (Vec<Lock>, Vec<Key>)> {
        let (input, lockkeys) = multi::separated_list1(tag("\n"), branch::alt((lock, key)))(input)?;
        let (locks, keys) =
            lockkeys
                .into_iter()
                .fold((vec![], vec![]), |(mut locks, mut keys), lockkey| {
                    match lockkey {
                        LockKey::Lock(lock) => locks.push(lock),
                        LockKey::Key(key) => keys.push(key),
                    };
                    (locks, keys)
                });
        Ok((input, (locks, keys)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Lock>, Vec<Key>)> {
        aoc::parse_with!(all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let (locks, keys) = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(locks.len(), 2);
    assert_eq!(keys.len(), 3);
    Ok(())
}
