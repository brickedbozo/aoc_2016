#![allow(dead_code, unused, clippy::all)]
use std::fmt::format;

use md5;
fn main() {
    let i1 = "abc";
    let i2 = "ugkcyxxp";
    // let p1 = part_1(i2);
    // println!("{:?}", p1);
    let p2 = part_2(i2);
    println!("{:?}", p2);
}
fn part_2(inp: &str) -> String {
    let mut res: [Option<char>; 8] = [None; 8];
    let mut i = 0_u64;
    let mut j = 0;
    while i < u64::MAX {
        if j == 8 {
            break;
        }
        let a = format!("{}{}", inp, i);
        let h = md5::compute(a.as_bytes());
        let h = format!("{:X}", h);
        if h.starts_with("00000") {
            let pos = h.chars().nth(5).unwrap();
            if pos.is_ascii_digit() {
                let pos = (pos as u8 - b'0') as usize;
                if pos < 8 {
                    let ch = h.chars().nth(6).unwrap();
                    if res[pos].is_none() {
                        res[pos] = Some(ch);
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    res.into_iter().map(|e| e.unwrap()).collect::<String>()
}
fn part_1(inp: &str) -> String {
    let mut res = String::new();
    let mut i = 0_u64;
    while i < u64::MAX {
        if res.len() == 8 {
            break;
        }
        let a = format!("{}{}", inp, i);
        let h = md5::compute(a.as_bytes());
        let h = format!("{:X}", h);
        if h.starts_with("00000") {
            res.push(h.chars().nth(5).unwrap());
        }
        i += 1;
    }

    res
}
