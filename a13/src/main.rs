extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("test.txt").unwrap();
    let mut sum = 0;
    for (i, j) in input.split("\n\n").enumerate() {
        let (left, right) = j.split_once('\n').unwrap();
        if compare_packets(left, right) {
            sum += i + 1;
        }
    }
    println!("pt1: {}", sum);
    let mut packets = vec![];
    for line in input.lines() {
        if line != "" {
            packets.push(parse_entry(line))
        }
    }
    let marker_1 = marker(2);
    let marker_2 = marker(6);
    packets.push(marker_1.clone());
    packets.push(marker_2.clone());
    packets.sort();
    let v1 = packets.iter().position(|x| *x == marker_1).unwrap() as i32 + 1;
    let v2 = packets.iter().position(|x| *x == marker_2).unwrap() as i32 + 1;
    println!("pt2: {:?}", v1 * v2)
}

fn marker(i: i32) -> Packet {
    Packet::List(vec![Packet::List(vec![Packet::Int(i)])])
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => {
                for i in 0..l0.len() {
                    if r0.get(i) == None {
                        return Ordering::Greater;
                    }
                    match l0[i].cmp(&r0[i]) {
                        Ordering::Equal => (),
                        x => return x,
                    }
                }
                if l0.len() == r0.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            (Self::List(_), Self::Int(r0)) => self.cmp(&Self::List(vec![Packet::Int(*r0)])),
            (Self::Int(l0), Self::List(_)) => Self::List(vec![Packet::Int(l0.clone())]).cmp(other),
            (Self::Int(l0), Self::Int(r0)) => l0.cmp(r0),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_value(value: Value) -> Packet {
    match value {
        Value::Number(i) => Packet::Int(i.as_i64().unwrap() as i32),
        Value::Array(arr) => Packet::List(arr.iter().map(|x| parse_value(x.clone())).collect()),
        _ => panic!(),
    }
}

fn parse_entry(entry: &str) -> Packet {
    let v: Value = serde_json::from_str(entry).unwrap();
    match v {
        Value::Number(i) => Packet::Int(i.as_i64().unwrap() as i32),
        Value::Array(arr) => Packet::List(arr.iter().map(|x| parse_value(x.clone())).collect()),
        _ => panic!(),
    }
}

fn compare_packets(a: &str, b: &str) -> bool {
    let a = parse_entry(a);
    let b = parse_entry(b);

    match a.cmp(&b) {
        Ordering::Less => true,
        Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let mut sum = 0;
        for (i, j) in input.split("\n\n").enumerate() {
            let (left, right) = j.split_once('\n').unwrap();
            if compare_packets(left, right) {
                sum += i + 1;
            }
        }
        assert_eq!(sum, 13);
    }

    #[test]
    fn part_2_works() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let mut packets = vec![];
        for line in input.lines() {
            if line != "" {
                packets.push(parse_entry(line))
            }
        }
        let marker_1 = marker(2);
        let marker_2 = marker(6);
        packets.push(marker_1.clone());
        packets.push(marker_2.clone());
        packets.sort();
        let v1 = packets.iter().position(|x| *x == marker_1).unwrap() as i32 + 1;
        let v2 = packets.iter().position(|x| *x == marker_2).unwrap() as i32 + 1;
        assert_eq!(v1 * v2, 140);
    }
}
