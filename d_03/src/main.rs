#![allow(unused, dead_code)]

use std::fs;
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = part_1(&inp);
    let p2 = part_2(&inp);
    println!("{p1}");
    println!("{p2}");
}

fn part_2(inp: &str) -> u64 {
    let mut res = 0;
    let mut list = parse_input(inp);
    let mut y = 0;
    let rows = list.len();
    while y <= rows - 3 {
        for x in 0..3 {
            let mut t = [list[y][x], list[y + 1][x], list[y + 2][x]];
            t.sort();
            if t[2] < t[0] + t[1] {
                res += 1;
            }
        }
        y += 3;
    }
    res
}
fn part_1(inp: &str) -> u64 {
    let mut res = 0;
    let mut list = parse_input(inp);
    for mut l in list.into_iter() {
        l.sort();
        if l[2] < l[0] + l[1] {
            res += 1;
        }
    }
    res
}
fn parse_input(inp: &str) -> Vec<Vec<u64>> {
    let mut list = Vec::new();
    for l in inp.lines() {
        let nums = l
            .trim()
            .replace("    ", ",")
            .replace("   ", ",")
            .replace("  ", ",")
            .replace(" ", ",")
            .split_terminator(",")
            .map(parse_str_to_number)
            .collect::<Vec<u64>>();
        list.push(nums);
    }
    list
}

fn parse_str_to_number(s: &str) -> u64 {
    let mut res = 0;
    let mut div = 1;
    for c in s.chars().rev() {
        let n = (c as u8) - b'0';
        res += n as u64 * div;
        div *= 10;
    }
    res
}
