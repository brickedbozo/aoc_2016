#![allow(dead_code, unused)]
use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("i2").unwrap();
    let input = input.trim().split_terminator(", ").collect::<Vec<&str>>();
    let p1 = part_1(&input);
    let p2 = part_2(&input);
    println!("{}", p1);
    println!("{}", p2);
}
fn part_1(inp: &[&str]) -> u64 {
    let mut pos = (0, 0);
    let mut dir = 'N';
    'main: for e in inp.iter() {
        match e.chars().next().unwrap() {
            'R' => match dir {
                'N' => dir = 'W',
                'W' => dir = 'S',
                'S' => dir = 'E',
                'E' => dir = 'N',
                _ => {}
            },
            'L' => match dir {
                'N' => dir = 'E',
                'W' => dir = 'N',
                'S' => dir = 'W',
                'E' => dir = 'S',
                _ => {}
            },
            _ => {}
        }
        let n = parse_str_to_number(&e[1..]) as i64;
        match dir {
            'N' => pos = (pos.0, pos.1 + n),
            'W' => pos = (pos.0 + n, pos.1),
            'S' => pos = (pos.0, pos.1 - n),
            'E' => pos = (pos.0 - n, pos.1),
            _ => {}
        }
    }
    calc_distance(pos)
}
fn part_2(inp: &[&str]) -> u64 {
    let mut res = 0;
    let mut pos = (0, 0);
    let mut pos_hs: HashSet<(i64, i64)> = HashSet::new();
    pos_hs.insert(pos);
    let mut dir = 'N';
    'main: for e in inp.iter() {
        match e.chars().next().unwrap() {
            'R' => match dir {
                'N' => dir = 'W',
                'W' => dir = 'S',
                'S' => dir = 'E',
                'E' => dir = 'N',
                _ => {}
            },
            'L' => match dir {
                'N' => dir = 'E',
                'W' => dir = 'N',
                'S' => dir = 'W',
                'E' => dir = 'S',
                _ => {}
            },
            _ => {}
        }
        let n = parse_str_to_number(&e[1..]) as i64;
        match dir {
            'N' => {
                let mut i = 0;
                while i < n {
                    pos = (pos.0, pos.1 + 1);
                    let is_unique = pos_hs.insert(pos);
                    if !is_unique {
                        res = calc_distance(pos);
                        break 'main;
                    }
                    i += 1;
                }
            }
            'W' => {
                let mut i = 0;
                while i < n {
                    pos = (pos.0 + 1, pos.1);
                    let is_unique = pos_hs.insert(pos);
                    if !is_unique {
                        res = calc_distance(pos);
                        break 'main;
                    }
                    i += 1;
                }
            }
            'S' => {
                let mut i = 0;
                while i < n {
                    pos = (pos.0, pos.1 - 1);
                    let is_unique = pos_hs.insert(pos);
                    if !is_unique {
                        res = calc_distance(pos);
                        break 'main;
                    }
                    i += 1;
                }
            }
            'E' => {
                let mut i = 0;
                while i < n {
                    pos = (pos.0 - 1, pos.1);
                    let is_unique = pos_hs.insert(pos);
                    if !is_unique {
                        res = calc_distance(pos);
                        break 'main;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
    }
    res
}
fn calc_distance(p: (i64, i64)) -> u64 {
    (p.0.abs() + p.1.abs()) as u64
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
