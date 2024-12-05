#![allow(dead_code, unused, clippy::all)]

use std::{collections::HashMap, fs};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("{:?}", p1);
    let p2 = calc_p2(&inp);
    println!("{:?}", p2);
}
fn calc_p2(inp: &str) -> String {
    let mut res = String::new();
    let msgs = parse_input(inp);
    let mut i = 0;
    while i < msgs[0].len() {
        let mut char_map: HashMap<char, u64> = HashMap::new();
        let mut j = 0;
        while j < msgs.len() {
            char_map
                .entry(msgs[j][i])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            j += 1;
        }
        let mut most_common = ('-', u64::MAX);
        char_map.into_iter().for_each(|(c, n)| {
            if most_common.1 > n {
                most_common = (c, n);
            }
        });
        res.push(most_common.0);
        i += 1;
    }

    res
}
fn calc_p1(inp: &str) -> String {
    let mut res = String::new();
    let msgs = parse_input(inp);
    let mut i = 0;
    while i < msgs[0].len() {
        let mut char_map: HashMap<char, u64> = HashMap::new();
        let mut j = 0;
        while j < msgs.len() {
            char_map
                .entry(msgs[j][i])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            j += 1;
        }
        let mut most_common = ('-', 0);
        char_map.into_iter().for_each(|(c, n)| {
            if most_common.1 < n {
                most_common = (c, n);
            }
        });
        res.push(most_common.0);
        i += 1;
    }

    res
}
fn parse_input(inp: &str) -> Vec<Vec<char>> {
    let mut res = vec![];
    for line in inp.lines() {
        let mut char_line = vec![];
        for c in line.chars() {
            char_line.push(c);
        }
        res.push(char_line);
    }

    res
}
