// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day22::*;

use std::collections::HashMap;

pub type Key = [i8; 4];

pub fn key_push(key: &mut Key, value: i8) {
    for j in 0..key.len() - 1 {
        key[j] = key[j + 1];
    }
    key[key.len() - 1] = value;
}

#[test]
fn test_key_push() {
    let mut k = Key::default();
    key_push(&mut k, -1);
    key_push(&mut k, -1);
    key_push(&mut k, 0);
    key_push(&mut k, 2);
    assert_eq!(k, [-1, -1, 0, 2]);
}

pub fn banana_for(mut secret: Num) -> HashMap<Key, usize> {
    let mut key = Key::default();
    let mut lastprice = (secret % 10) as i8;
    let mut data = HashMap::<Key, usize>::default();
    for i in 0..2000 {
        secret = evolve(secret);
        let price = (secret % 10) as i8;
        key_push(&mut key, price - lastprice);
        if i > 2 {
            let _ = data.entry(key).or_insert(price as usize);
        }
        lastprice = price;
    }
    data
}

pub fn banana_update(secret: Num, bananas: &mut HashMap<Key, usize>) {
    for (k, v) in banana_for(secret).into_iter() {
        let e = bananas.entry(k).or_default();
        *e += v;
    }
}

#[test]
fn test1() {
    let mut bananas = Default::default();
    banana_update(1, &mut bananas);
    assert_eq!(bananas[&[-2, 1, -1, 3]], 7);
}

#[test]
fn test2() {
    let mut bananas = Default::default();
    banana_update(2, &mut bananas);
    assert_eq!(bananas[&[-2, 1, -1, 3]], 7);
}

#[test]
fn test3() {
    let mut bananas = Default::default();
    banana_update(3, &mut bananas);
    assert!(!bananas.contains_key(&[-2, 1, -1, 3]));
}

#[test]
fn test2024() {
    let mut bananas = Default::default();
    banana_update(2024, &mut bananas);
    assert_eq!(bananas[&[-2, 1, -1, 3]], 9);
}

#[test]
fn test_sum() {
    let mut bananas = Default::default();
    banana_update(1, &mut bananas);
    banana_update(2, &mut bananas);
    banana_update(3, &mut bananas);
    banana_update(2024, &mut bananas);
    assert_eq!(bananas[&[-2, 1, -1, 3]], 23);
    assert_eq!(bananas.into_values().max().unwrap(), 23);
}

#[test]
fn test_extra1() {
    let mut bananas = Default::default();
    banana_update(2021, &mut bananas);
    banana_update(5017, &mut bananas);
    banana_update(19751, &mut bananas);
    assert_eq!(bananas.into_values().max().unwrap(), 27);
}

#[test]
fn test_extra2() {
    let mut bananas = Default::default();
    banana_update(5053, &mut bananas);
    banana_update(10083, &mut bananas);
    banana_update(11263, &mut bananas);
    assert_eq!(bananas.into_values().max().unwrap(), 27);
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let bananas = input
        .into_iter()
        .fold(Default::default(), |mut data, secret0| {
            banana_update(secret0, &mut data);
            data
        });
    Ok(bananas.into_values().max().unwrap())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
