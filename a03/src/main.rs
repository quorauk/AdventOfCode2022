use std::{fs::File, io::Read};

struct Rucksack {
    left: String,
    right: String,
}
struct Item {
    item: Option<char>,
}

struct Group<'a> {
    elves: Vec<&'a str>,
}

impl Rucksack {
    fn new(rucksack_data: String) -> Self {
        let (left, right) = rucksack_data.split_at(rucksack_data.len() / 2);
        Rucksack {
            left: String::from(left),
            right: String::from(right),
        }
    }

    fn shared_item(&self) -> Item {
        for i in self.left.chars() {
            if self.right.contains(i) {
                return Item { item: Some(i) };
            }
        }
        Item { item: None }
    }
}

impl Item {
    fn priority(&self) -> u32 {
        if let Some(i) = self.item {
            if i.is_uppercase() {
                return i.to_digit(36).unwrap() + 17;
            }
            return i.to_digit(36).unwrap() - 9;
        }
        0
    }
}

impl Group<'_> {
    fn new<'a>(group_data: &'a str) -> Group<'a> {
        Group {
            elves: group_data.split('\n').collect(),
        }
    }

    fn shared_item(&self) -> Item {
        if self.elves.len() < 3 {
            return Item { item: None };
        }
        let elf_a = self.elves.get(0).unwrap();
        let elf_b = self.elves.get(1).unwrap();
        let elf_c = self.elves.get(2).unwrap();
        let a_b_share: String = elf_a
            .chars()
            .filter(|c| elf_b.chars().find(|b| *c == *b).is_some())
            .collect();
        let b_c_share: String = elf_c
            .chars()
            .filter(|c| elf_b.chars().find(|b| *b == *c).is_some())
            .collect();
        for item in a_b_share.chars() {
            for possible in b_c_share.chars() {
                if item == possible {
                    return Item { item: Some(item) };
                }
            }
        }

        Item { item: None }
    }
}

fn main() {
    let file = File::open("./input.txt");
    let mut file = match file {
        Err(_) => return,
        Ok(file) => file,
    };

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    println!(
        "{}",
        contents
            .split("\n")
            .into_iter()
            .map(|s| Rucksack::new(s.to_string()).shared_item().priority())
            .sum::<u32>()
    );

    let file = File::open("./input.txt");
    let mut file = match file {
        Err(_) => return,
        Ok(file) => file,
    };

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut groups = Vec::new();
    for chunk in lines.chunks(3) {
        groups.push(Group::new(chunk.join("\n").as_str()).shared_item());
    }
    println!("{}", groups.iter().map(|g| g.priority()).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksacks() {
        let r = Rucksack::new(String::from("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(r.shared_item().priority(), 16);
        let r = Rucksack::new(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!(r.shared_item().priority(), 38)
    }

    #[test]
    fn items() {
        let r = Item { item: Some('p') };
        assert_eq!(r.priority(), 16);
        let r = Item { item: Some('L') };
        assert_eq!(r.priority(), 38);
    }

    #[test]
    fn groups() {
        let r = Group::new(
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg",
        );
        assert_eq!(r.shared_item().priority(), 18);
        let r = Group::new(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n",
        );
        assert_eq!(r.shared_item().priority(), 52);
    }
}
