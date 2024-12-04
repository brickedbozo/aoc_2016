#![allow(dead_code, unused)]
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
    mem::replace,
};
#[derive(Debug)]
struct Room1 {
    map: HashMap<char, u64>,
    id: u64,
    checksum: String,
}
#[derive(Debug)]
struct Room2 {
    name: String,
    id: u64,
}
fn main() {
    let inp = fs::read_to_string("i2").unwrap();
    let p1 = calc_part1(&inp);
    let p2 = calc_part2(&inp);
    println!("{p1}");
    for l in p2 {
        if l.contains("north") {
            println!("{:?}", l);
        }
    }
}
fn calc_part2(inp: &str) -> Vec<String> {
    let mut res = vec![];
    let rooms = parse_input_p2(inp);
    for room in rooms {
        let mut decrypted_name = String::new();
        for c in room.name.chars() {
            if c.is_ascii_alphabetic() {
                let mut decrypted_char = c;
                let mut i = 0;
                while i < room.id {
                    if decrypted_char as u8 == 122 {
                        decrypted_char = 'a';
                        i += 1;
                        continue;
                    }
                    decrypted_char = (decrypted_char as u8 + 1) as char;
                    i += 1;
                }
                decrypted_name.push(decrypted_char);
            } else {
                decrypted_name.push(' ');
            }
        }
        decrypted_name += " : ";
        decrypted_name += &room.id.to_string();
        res.push(decrypted_name);
    }
    res
}
fn calc_part1(inp: &str) -> u64 {
    let mut res = 0;
    let rooms = parse_input_p1(inp);
    'main: for room in rooms {
        let mut checksum_vals = vec![];
        for c in room.checksum.chars() {
            let r = room.map.get(&c);
            match r {
                Some(e) => checksum_vals.push(*e),
                None => continue 'main,
            }
        }
        let mut char_ord = Vec::new();
        for r in &room.map {
            char_ord.push(*r.1);
        }
        char_ord.sort_by(|l, r| r.cmp(l));
        if checksum_vals == char_ord[0..5] {
            res += room.id;
        }
    }
    res
}

fn parse_input_p2(inp: &str) -> Vec<Room2> {
    let mut rooms = vec![];
    for l in inp.lines() {
        let id = l
            .chars()
            .rev()
            .skip(7)
            .take(3)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let id = parse_str_to_number(&id);
        let mut map: HashMap<char, u64> = HashMap::new();
        let name = l.chars().rev().skip(11).collect::<String>();
        let name = name.replace("-", " ").chars().rev().collect::<String>();

        let room = Room2 { name, id };
        rooms.push(room);
    }
    rooms
}
fn parse_input_p1(inp: &str) -> Vec<Room1> {
    let mut rooms = vec![];
    for l in inp.lines() {
        let checksum = l
            .chars()
            .rev()
            .skip(1)
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let id = l
            .chars()
            .rev()
            .skip(7)
            .take(3)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let id = parse_str_to_number(&id);
        let mut map: HashMap<char, u64> = HashMap::new();
        l.chars()
            .rev()
            .skip(11)
            .filter(|e| e.is_ascii_alphabetic())
            .for_each(|c| {
                map.entry(c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            });
        let room = Room1 { map, id, checksum };
        rooms.push(room);
    }
    rooms
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
