#![allow(dead_code, unused, clippy::all)]

use std::{array, fs};
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_p1(&inp);
    println!("{p1}");
    let p2 = calc_p2(&inp);
    println!("{p2}");
}
fn calc_p2(inp: &str) -> u64 {
    let mut res = 0;
    let addr_list = parse_input(inp);
    for addr in addr_list {
        if check_if_supports_ssl(&addr) {
            res += 1;
        }
    }
    res
}
fn calc_p1(inp: &str) -> u64 {
    let mut res = 0;
    let addr_list = parse_input(inp);
    for addr in addr_list {
        if check_if_supports_tls(&addr) {
            res += 1;
        }
    }
    res
}
fn check_if_supports_tls(addr: &Vec<String>) -> bool {
    let mut seq1 = vec![];
    let mut seq2 = vec![];
    for (i, s) in addr.iter().enumerate() {
        if i % 2 == 0 {
            seq1.push(s);
        } else {
            seq2.push(s);
        }
    }
    let mut s1 = false;
    let mut s2 = false;
    for s in seq1 {
        if check_if_contains_abba(s) {
            s1 = true;
            break;
        }
    }
    for s in seq2 {
        if check_if_contains_abba(s) {
            s2 = true;
            break;
        }
    }
    if s1 && !s2 {
        return true;
    }
    false
}
fn check_if_supports_ssl(addr: &Vec<String>) -> bool {
    let mut seq1 = vec![];
    let mut seq2 = vec![];
    for (i, s) in addr.iter().enumerate() {
        if i % 2 == 0 {
            seq1.push(s.clone());
        } else {
            seq2.push(s.clone());
        }
    }
    let mut seq1_all_aba = vec![];
    for s in seq1 {
        let aba_vec = get_all_aba(&s);
        if !aba_vec.is_empty() {
            for aba in aba_vec {
                seq1_all_aba.push(aba.to_owned());
            }
        }
    }
    let mut seq2_all_aba = vec![];
    for s2 in seq2 {
        let a = get_all_aba(&s2);
        if !a.is_empty() {
            for a1 in a {
                seq2_all_aba.push(a1.to_owned());
            }
        }
    }
    for s1_aba in &seq1_all_aba {
        for s2_aba in &seq2_all_aba {
            let a1 = s2_aba.chars().nth(1).unwrap();
            let a2 = s2_aba.chars().nth(0).unwrap();
            let a3 = s2_aba.chars().nth(1).unwrap();
            let s2_aba_mod = format!("{}{}{}", a1, a2, a3);
            if s1_aba == &s2_aba_mod {
                return true;
            }
        }
    }

    false
}
fn get_all_aba(s: &str) -> Vec<&str> {
    let mut res = vec![];
    let l = s.len();
    let mut i = 0;
    while i + 2 < l {
        let n1 = s.chars().nth(i).unwrap();
        let n2 = s.chars().nth(i + 1).unwrap();
        let n3 = s.chars().nth(i + 2).unwrap();
        if n1 == n3 && n1 != n2 {
            res.push(&s[i..=i + 2]);
        }
        i += 1;
    }
    res
}
fn check_if_contains_abba(s: &str) -> bool {
    let l = s.len();
    let mut i = 0;
    while i + 3 < l {
        let sl = &s[i..=i + 3];
        if sl.chars().nth(0) == sl.chars().nth(3)
            && sl.chars().nth(1) == sl.chars().nth(2)
            && sl.chars().nth(0) != sl.chars().nth(1)
        {
            return true;
        }
        i += 1;
    }
    false
}
fn parse_input(inp: &str) -> Vec<Vec<String>> {
    let mut res = vec![];
    for l in inp.lines() {
        let a = l
            .replace("[", ",")
            .replace("]", ",")
            .split_terminator(",")
            .map(|e| e.to_owned())
            .collect::<Vec<String>>();
        res.push(a);
    }
    res
}
