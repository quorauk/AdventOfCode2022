use regex::Regex;
use std::{fs::File, io::Read};

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    you: RPS,
    opponent: RPS,
}

struct Round2 {
    opponent: RPS,
    result: RoundResult,
}

enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl Round {
    pub fn score(&self) -> i32 {
        let hand_score = match self.you {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let outcome_score = match self.result() {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };

        hand_score + outcome_score
    }

    fn result(&self) -> RoundResult {
        match self.you {
            RPS::Rock => match self.opponent {
                RPS::Scissors => RoundResult::Win,
                RPS::Paper => RoundResult::Loss,
                RPS::Rock => RoundResult::Draw,
            },
            RPS::Paper => match self.opponent {
                RPS::Scissors => RoundResult::Loss,
                RPS::Paper => RoundResult::Draw,
                RPS::Rock => RoundResult::Win,
            },
            RPS::Scissors => match self.opponent {
                RPS::Scissors => RoundResult::Draw,
                RPS::Paper => RoundResult::Win,
                RPS::Rock => RoundResult::Loss,
            },
        }
    }
}

impl Round2 {
    pub fn score(&self) -> i32 {
        let hand_score = match self.result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };

        let outcome_score = match self.needed_shape() {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        hand_score + outcome_score
    }

    fn needed_shape(&self) -> RPS {
        match self.opponent {
            RPS::Rock => match self.result {
                RoundResult::Win => RPS::Paper,
                RoundResult::Loss => RPS::Scissors,
                RoundResult::Draw => RPS::Rock,
            },
            RPS::Paper => match self.result {
                RoundResult::Win => RPS::Scissors,
                RoundResult::Loss => RPS::Rock,
                RoundResult::Draw => RPS::Paper,
            },
            RPS::Scissors => match self.result {
                RoundResult::Win => RPS::Rock,
                RoundResult::Loss => RPS::Paper,
                RoundResult::Draw => RPS::Scissors,
            },
        }
    }
}

fn main() {
    let file_res = File::open("./input.txt");
    if file_res.is_err() {
        panic!("file not found");
    }
    let mut file = file_res.unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let rounds: i32 = contents
        .as_str()
        .split("\n")
        .into_iter()
        .map(|x| str_to_round(x))
        .map(|x| match x {
            Some(r) => r.score(),
            None => 0,
        })
        .sum();
    println!("{:?}", rounds);
    let rounds: i32 = contents
        .as_str()
        .split("\n")
        .into_iter()
        .map(|x| str_to_round_2(x))
        .map(|x| match x {
            Some(r) => r.score(),
            None => 0,
        })
        .sum();
    println!("{:?}", rounds)
}

fn str_to_round(string: &str) -> Option<Round> {
    let re = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    if re.is_match(string) {
        let captures = re.captures(string);
        if let Some(captures) = captures {
            let opponent = match captures.get(1).unwrap().as_str() {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!(),
            };
            let you = match captures.get(2).unwrap().as_str() {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                _ => panic!(),
            };
            return Some(Round { you, opponent });
        }
    }
    None
}

fn str_to_round_2(string: &str) -> Option<Round2> {
    let re = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    if re.is_match(string) {
        let captures = re.captures(string);
        if let Some(captures) = captures {
            let opponent = match captures.get(1).unwrap().as_str() {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!(),
            };
            let result = match captures.get(2).unwrap().as_str() {
                "X" => RoundResult::Loss,
                "Y" => RoundResult::Draw,
                "Z" => RoundResult::Win,
                _ => panic!(),
            };
            return Some(Round2 { result, opponent });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::Round;
    use crate::Round2;

    #[test]
    fn it_works() {
        let round = Round {
            opponent: crate::RPS::Rock,
            you: crate::RPS::Paper,
        };
        assert_eq!(round.score(), 8);
    }

    #[test]
    fn it_works_round_2() {
        let round = Round2 {
            opponent: crate::RPS::Rock,
            result: crate::RoundResult::Draw,
        };
        assert_eq!(round.score(), 4);
    }
}
