use std::{fs::File, io::Read, vec};

use regex::Regex;

struct WeirdCPU {
    x: i32,
    cycle: i32,
    signal_strength: i32,
}

impl WeirdCPU {
    fn new() -> Self {
        WeirdCPU {
            cycle: 0,
            x: 1,
            signal_strength: 0,
        }
    }

    fn run(&mut self, cmd: Command) {
        match cmd {
            Command::Noop => self.next_cycle(),
            Command::Add(amount) => {
                self.next_cycle();
                self.next_cycle();
                self.x += amount;
            }
        }
    }

    fn next_cycle(&mut self) {
        if &self.cycle % 40 == 0 {
            print!("\n");
        }
        if (self.cycle % 40) >= self.x - 1 && (self.cycle % 40) <= self.x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        self.cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&self.cycle) {
            self.signal_strength += self.cycle * self.x
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Noop,
    Add(i32),
}

fn load_instructions(filename: String) -> Vec<Command> {
    let file = File::open(filename);
    let mut contents = String::new();
    let _ = file.unwrap().read_to_string(&mut contents);
    let noop_re = Regex::new(r"noop").unwrap();
    let addx_re = Regex::new(r"addx (-?\d+)").unwrap();
    let mut commands = vec![];
    for line in contents.split("\n") {
        if noop_re.is_match(line) {
            commands.push(Command::Noop)
        } else if let Some(cap) = addx_re.captures(line) {
            if let Ok(val) = cap[1].parse() {
                commands.push(Command::Add(val))
            }
        }
    }
    commands
}

fn main() {
    let instructions = load_instructions("input.txt".to_string());
    let mut cpu = WeirdCPU::new();
    for instruction in instructions {
        cpu.run(instruction)
    }
    println!("pt1: {}", cpu.signal_strength);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_code() {
        let instructions = load_instructions("test_2.txt".to_string());
        let mut cpu = WeirdCPU::new();
        for instruction in instructions {
            cpu.run(instruction)
        }
        assert_eq!(cpu.signal_strength, 13140);
    }
}
