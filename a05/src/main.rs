use regex::Regex;
use std::{fs::File, io::Read};

fn read_line(str: String) -> Vec<String> {
    let re = Regex::new(r"[\[|\s](?P<val>.)[\]|\s]\s?").unwrap();
    re.captures_iter(&str)
        .map(|cap| cap.name("val").unwrap().as_str().to_string())
        .collect()
}

fn is_cargo_line(line: &str) -> bool {
    let re = Regex::new(r"^(\s{3,4}|\[\w\]\s?)*$").unwrap();
    re.is_match(&line)
}

fn is_move_line(line: &str) -> bool {
    let re = Regex::new(r"^move \d+ from \d to \d$").unwrap();
    re.is_match(&line)
}

fn read_stacks(str: &String) -> Vec<Vec<String>> {
    let mut stacks = Vec::new();

    for line in str.split('\n') {
        if is_cargo_line(&line) {
            stacks.push(read_line(line.to_string()))
        }
    }
    let mut new_stack = Vec::new();
    for i in 0..stacks.first().unwrap().len() {
        let mut new_vec = Vec::new();
        for v in &stacks {
            if let Some(val) = v.get(i) {
                if val != " " {
                    new_vec.push(val.clone());
                }
            }
        }
        new_vec.reverse();
        new_stack.push(new_vec)
    }
    new_stack
}

#[derive(Debug, PartialEq)]
struct Move {
    pub x: usize,
    pub from: usize,
    pub to: usize,
}

fn read_instructions(str: &String) -> Vec<Move> {
    let mut instructions = Vec::new();
    for line in str.split('\n') {
        if is_move_line(line) {
            let re = Regex::new(r"^move (?P<x>\d+) from (?P<from>\d) to (?P<to>\d)$").unwrap();
            if let Some(cap) = re.captures(line) {
                let x = cap.name("x").unwrap().as_str().parse().unwrap();
                let from = cap.name("from").unwrap().as_str().parse().unwrap();
                let to = cap.name("to").unwrap().as_str().parse().unwrap();
                instructions.push(Move { x, from, to })
            }
        }
    }
    instructions
}

fn handle_instructions(str: &String) -> String {
    let mut stacks = read_stacks(str);
    let instructions = read_instructions(str);
    for instruction in instructions {
        let mut moving = Vec::new();
        let from = stacks.get_mut(instruction.from - 1).unwrap();
        for _ in 0..instruction.x {
            moving.push(from.pop().unwrap());
        }
        let to = stacks.get_mut(instruction.to - 1).unwrap();
        for cargo in moving {
            to.push(cargo)
        }
    }
    let tops: Vec<String> = stacks.iter().map(|x| x.last().unwrap().clone()).collect();
    tops.join("")
}

fn handle_instructions_9001(str: &String) -> String {
    let mut stacks = read_stacks(str);
    let instructions = read_instructions(str);
    for instruction in instructions {
        let mut moving = Vec::new();
        let from = stacks.get_mut(instruction.from - 1).unwrap();
        for _ in 0..instruction.x {
            moving.push(from.pop().unwrap());
        }
        moving.reverse();
        let to = stacks.get_mut(instruction.to - 1).unwrap();
        for cargo in moving {
            to.push(cargo)
        }
    }
    let tops: Vec<String> = stacks.iter().map(|x| x.last().unwrap().clone()).collect();
    tops.join("")
}

fn main() {
    let file = File::open("./input.txt");
    let mut file = match file {
        Err(_) => return,
        Ok(file) => file,
    };
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    println!("{:?}", handle_instructions(&contents));
    println!("{:?}", handle_instructions_9001(&contents));
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn can_read_lines() {
        assert_eq!(read_line("    [D]     ".to_string()), vec![" ", "D", " "])
    }

    #[test]
    fn can_read_stacks() {
        assert_eq!(
            read_stacks(&"    [D]    \n[N] [C]    \n[Z] [M] [P] \n 1   2   3 ".to_string()),
            vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]
        )
    }

    #[test]
    fn num_line() {
        assert_eq!(is_cargo_line("    [D]     "), true);

        assert_eq!(is_cargo_line(" 1   2   3 "), false);
    }

    #[test]
    fn test_read_move() {
        assert_eq!(
            *read_instructions(&String::from("move 1 from 2 to 1\n"))
                .first()
                .unwrap(),
            Move {
                x: 1,
                from: 2,
                to: 1
            }
        );
    }

    #[test]
    fn integrate() {
        let file = File::open("./test.txt");
        let mut file = match file {
            Err(_) => return,
            Ok(file) => file,
        };
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        assert_eq!(handle_instructions(&contents), "DCP");
    }
}
