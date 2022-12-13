use std::{fs::File, io::Read};

use regex::Regex;

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    visited: Vec<(i32, i32)>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Rope {
            visited: Vec::new(),
            knots: vec![(0, 0); knots],
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Instruction {
    dir: Dir,
    steps: i32,
}

fn load_instructions(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename);
    let mut contents = String::new();
    let _ = file.unwrap().read_to_string(&mut contents);
    let line_re = Regex::new(r"([A-Z]) (\d+)").unwrap();
    let mut instructions = Vec::new();
    for cap in line_re.captures_iter(&contents) {
        let dir = if cap[1].to_string() == "D".to_string() {
            Dir::D
        } else if cap[1].to_string() == "U".to_string() {
            Dir::U
        } else if cap[1].to_string() == "L".to_string() {
            Dir::L
        } else if cap[1].to_string() == "R".to_string() {
            Dir::R
        } else {
            panic!()
        };
        instructions.push(Instruction {
            dir,
            steps: cap[2].parse::<i32>().unwrap(),
        })
    }
    instructions
}

impl Rope {
    fn feed_instruction(&mut self, instruction: Instruction) {
        for _ in 0..instruction.steps {
            match instruction.dir {
                Dir::U => self.knots[0] = (self.knots[0].0, self.knots[0].1 + 1),
                Dir::D => self.knots[0] = (self.knots[0].0, self.knots[0].1 - 1),
                Dir::L => self.knots[0] = (self.knots[0].0 - 1, self.knots[0].1),
                Dir::R => self.knots[0] = (self.knots[0].0 + 1, self.knots[0].1),
            }
            self.knots_follow();
        }
    }

    fn knots_follow(&mut self) {
        let mut knots = self.knots.clone();
        let knot_len = knots.len() - 1;
        for i in 0..knot_len {
            let head = knots[i];
            let mut tail = knots[i + 1];

            if head.0.abs_diff(tail.0) + head.1.abs_diff(tail.1) > 3 {
                tail.0 = head.0 + (tail.0 - head.0).clamp(-1, 1);
                tail.1 = head.1 + (tail.1 - head.1).clamp(-1, 1);
            } else if head.0.abs_diff(tail.0) + head.1.abs_diff(tail.1) == 3 {
                if head.0.abs_diff(tail.0) > 1 {
                    tail.0 = head.0 + (tail.0 - head.0).clamp(-1, 1);
                    tail.1 = head.1
                } else {
                    tail.0 = head.0;
                    tail.1 = head.1 + (tail.1 - head.1).clamp(-1, 1);
                }
            } else {
                tail.0 = head.0 + (tail.0 - head.0).clamp(-1, 1);
                tail.1 = head.1 + (tail.1 - head.1).clamp(-1, 1);
            }
            knots[i + 1] = tail;
            self.knots[i + 1] = tail;
            if i == knot_len - 1 {
                self.visited.push(tail)
            }
        }
    }
}

fn main() {
    let mut rope = Rope::new(2);
    let instructions = load_instructions("./input.txt");
    for instruction in instructions {
        rope.feed_instruction(instruction);
    }
    rope.visited.sort();
    rope.visited.dedup();
    println!("pt1: {:?}", rope.visited.len());
    let mut rope = Rope::new(10);
    let instructions = load_instructions("./input.txt");
    for instruction in instructions {
        rope.feed_instruction(instruction);
    }
    rope.visited.sort();
    rope.visited.dedup();
    println!("pt2: {:?}", rope.visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rope = Rope::new(2);
        let instructions = load_instructions("./test.txt");
        for instruction in instructions {
            rope.feed_instruction(instruction);
        }
        rope.visited.sort();
        rope.visited.dedup();
        assert_eq!(rope.visited.len(), 13);
        let mut rope = Rope::new(2);
        let instructions = load_instructions("./input.txt");
        for instruction in instructions {
            rope.feed_instruction(instruction);
        }
        rope.visited.sort();
        rope.visited.dedup();
        assert_eq!(rope.visited.len(), 5513);
        let mut rope = Rope::new(10);
        let instructions = load_instructions("./test.txt");
        for instruction in instructions {
            rope.feed_instruction(instruction);
        }
        rope.visited.sort();
        rope.visited.dedup();
        assert_eq!(rope.visited.len(), 1);
        let mut rope = Rope::new(10);
        let instructions = load_instructions("./test_2.txt");
        for instruction in instructions {
            rope.feed_instruction(instruction);
        }
        rope.visited.sort();
        rope.visited.dedup();
        assert_eq!(rope.visited.len(), 36)
    }

    #[test]
    fn test_follow_instruction() {
        let mut rope = Rope::new(2);
        rope.feed_instruction(Instruction {
            dir: Dir::R,
            steps: 4,
        });
        assert_eq!(rope.knots[1], (3, 0));
        rope.feed_instruction(Instruction {
            dir: Dir::U,
            steps: 1,
        });
        assert_eq!(rope.knots[1], (3, 0));
        rope.feed_instruction(Instruction {
            dir: Dir::U,
            steps: 1,
        });
        assert_eq!(rope.knots[1], (4, 1));
    }
}
