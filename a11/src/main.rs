#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    pos_monkey: usize,
    neg_monkey: usize,
    inspections: i64,
    test_relief: i64,
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(i64),
    Add(i64),
    Pow,
}

impl Monkey {
    fn do_monkey_business(&mut self, modulus: i64) -> (Vec<i64>, Vec<i64>) {
        let items = self.items.clone();
        let mut pos = vec![];
        let mut neg = vec![];

        for item in items {
            self.inspections += 1;
            let mut new = match self.operation {
                Operation::Mul(x) => item * x % modulus,
                Operation::Add(x) => item + x % modulus,
                Operation::Pow => item * item % modulus,
            };
            new = new / self.test_relief;
            if new % self.test == 0 {
                pos.push(new);
            } else {
                neg.push(new);
            }
        }
        self.items.clear();
        (pos, neg)
    }
}

fn parse_monkey(monkey: &str, test_relief: i64) -> Monkey {
    let lines: Vec<&str> = monkey.split('\n').collect();
    let items = lines[1][18..]
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
    let test = lines[3][21..].parse().unwrap();
    let pos_monkey = lines[4][29..].parse().unwrap();
    let neg_monkey = lines[5][30..].parse().unwrap();

    let operation = if &lines[2][19..] == "old * old" {
        Operation::Pow
    } else {
        let arg: i64 = lines[2][25..].parse().unwrap();
        if lines[2].chars().nth(23).unwrap() == '*' {
            Operation::Mul(arg)
        } else {
            Operation::Add(arg)
        }
    };
    Monkey {
        items,
        operation,
        test,
        pos_monkey,
        neg_monkey,
        inspections: 0,
        test_relief,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|x| parse_monkey(x, 3)).collect();
    let modulus = monkeys.clone().iter().fold(1, |acc, m| acc * m.test);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (p, n) = monkeys[i].do_monkey_business(modulus);
            let pos_monkey = monkeys[i].pos_monkey;
            let neg_monkey = monkeys[i].neg_monkey;
            monkeys[pos_monkey].items.extend(p);
            monkeys[neg_monkey].items.extend(n);
        }
    }
    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, monkeys[i].inspections
        )
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    println!("pt1: {}", monkeys[0].inspections * monkeys[1].inspections);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|x| parse_monkey(x, 1)).collect();
    let monkey_len = monkeys.len();
    let modulus = monkeys.clone().iter().fold(1, |acc, m| acc * m.test);

    for round in 0..10000 {
        println!("{}", round);
        for i in 0..monkey_len {
            let (p, n) = monkeys[i].do_monkey_business(modulus);
            let pos_monkey = monkeys[i].pos_monkey;
            let neg_monkey = monkeys[i].neg_monkey;
            monkeys[pos_monkey].items.extend(p);
            monkeys[neg_monkey].items.extend(n);
        }
    }
    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, monkeys[i].inspections
        )
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    println!("pt2: {}", monkeys[0].inspections * monkeys[1].inspections);
}
