extern crate serde_json;
extern crate serde;

use std::cmp::Ordering;
use serde_json::Value;


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
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
    let marker_1 = Packet::List(
        vec![
            Packet::List(
                vec![Packet::Int(2)]
            )
        ]
    );
    let marker_2 = Packet::List(
        vec![
            Packet::List(
                vec![Packet::Int(6)]
            )
        ]
    );
    packets.push(marker_1.clone());
    packets.push(marker_2.clone());
    packets.sort();
    let v1 = packets.iter().position(|x| *x == marker_1).unwrap() as i32 + 1;
    let v2 = packets.iter().position(|x| *x == marker_2).unwrap() as i32 + 1;
    println!("pt2: {:?}", v1 * v2)
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32)
}

enum PacketCMP {
    Equal,
    Unsure,
    NotEqual
}

impl Packet {
    fn packet_cmp(&self, other: &Self) -> PacketCMP {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => {
                for i in 0..l0.len() {
                    if r0.get(i) == None {
                        return PacketCMP::NotEqual
                    }
                    match l0[i].packet_cmp(&r0[i]) {
                        PacketCMP::Equal => return PacketCMP::Equal,
                        PacketCMP::Unsure => (),
                        PacketCMP::NotEqual => return PacketCMP::NotEqual,
                    }
                }
                if l0.len() == r0.len() {
                    PacketCMP::Unsure
                } else {
                    PacketCMP::Equal
                }
            },
            (Self::List(_), Self::Int(r0)) => self.packet_cmp(&Self::List(vec![Packet::Int(*r0)])),
            (Self::Int(l0), Self::List(_)) => Self::List(vec![Packet::Int(l0.clone())]).packet_cmp(other),
            (Self::Int(l0), Self::Int(r0)) => {
                if l0 < r0 {
                    PacketCMP::Equal
                } else if l0 ==  r0 {
                    PacketCMP::Unsure
                } else {
                    PacketCMP::NotEqual
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.packet_cmp(other) {
            PacketCMP::Equal => Some(Ordering::Less),
            PacketCMP::Unsure => Some(Ordering::Equal),
            PacketCMP::NotEqual => Some(Ordering::Greater),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.packet_cmp(other) {
            PacketCMP::Equal => Ordering::Less,
            PacketCMP::Unsure => Ordering::Equal,
            PacketCMP::NotEqual => Ordering::Greater,
        }
    }

}

fn parse_value(value: Value) -> Packet {
    match value {
        Value::Number(i) => Packet::Int(i.as_i64().unwrap() as i32),
        Value::Array(arr) => Packet::List(arr.iter().map(|x| parse_value(x.clone())).collect()),
        _ => panic!()
    }
}


fn parse_entry(entry: &str) -> Packet {
    let v: Value = serde_json::from_str(entry).unwrap();
    match v {
        Value::Number(i) => Packet::Int(i.as_i64().unwrap() as i32),
        Value::Array(arr) => Packet::List(arr.iter().map(|x| parse_value(x.clone())).collect()),
        _ => panic!()
    }

}

fn compare_packets(a: &str, b: &str) -> bool {
    let a = parse_entry(a);
    let b = parse_entry(b);

    match a.packet_cmp(&b) {
        PacketCMP::Equal => true,
        PacketCMP::Unsure => true,
        PacketCMP::NotEqual => false,
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(
        compare_packets(
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
        ),
        true
    );
    assert_eq!(
        compare_packets(
            "[1,[2,3,4]]",
            "[[1],4]",
        ),
        true
    );
    assert_eq!(
        compare_packets(
            "[7,7,7,7]",
            "[7,7,7]",
        ),
        false
    )
  }
}
