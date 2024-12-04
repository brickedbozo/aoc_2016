#![allow(dead_code, unused)]

use std::{collections::HashMap, fs};
fn main() {
    let inp = fs::read_to_string("i1").unwrap();
    let p1 = part_1(&inp);
    let p2 = part_2(&inp);
    println!("{:?}", p1);
    println!("{:?}", p2);
}
fn part_2(inp: &str) -> String {
    let mut keypad: HashMap<(u8, u8), char> = HashMap::with_capacity(13);
    let mut visited_numbers = vec![];
    keypad.insert((0, 2), '1');
    keypad.insert((1, 1), '2');
    keypad.insert((1, 2), '3');
    keypad.insert((1, 3), '4');
    keypad.insert((2, 0), '5');
    keypad.insert((2, 1), '6');
    keypad.insert((2, 2), '7');
    keypad.insert((2, 3), '8');
    keypad.insert((2, 4), '9');
    keypad.insert((3, 1), 'A');
    keypad.insert((3, 2), 'B');
    keypad.insert((3, 3), 'C');
    keypad.insert((4, 2), 'D');
    let (mut y, mut x) = (2, 0);
    for l in inp.lines() {
        for c in l.chars() {
            match c {
                'L' => {
                    if x > 0 && keypad.contains_key(&(y, x - 1)) {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < 5 && keypad.contains_key(&(y, x + 1)) {
                        x += 1;
                    }
                }
                'U' => {
                    if y > 0 && keypad.contains_key(&(y - 1, x)) {
                        y -= 1;
                    }
                }
                'D' => {
                    if y < 5 && keypad.contains_key(&(y + 1, x)) {
                        y += 1;
                    }
                }

                _ => {}
            }
        }
        visited_numbers.push(keypad.get(&(y, x)).unwrap());
    }
    visited_numbers.into_iter().collect::<String>()
}
fn part_1(inp: &str) -> u64 {
    let numpad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut visited_numbers = vec![];
    let (mut y, mut x) = (1, 1);
    for l in inp.lines() {
        for c in l.chars() {
            match c {
                'L' => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < 2 {
                        x += 1;
                    }
                }
                'U' => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                'D' => {
                    if y < 2 {
                        y += 1;
                    }
                }
                _ => {}
            }
        }
        visited_numbers.push(numpad[y][x]);
    }
    let mut div = 1;
    let mut res = 0;
    visited_numbers.into_iter().rev().for_each(|n| {
        res += n * div;
        div *= 10;
    });
    res
}
