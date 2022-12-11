use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Elf {
    pub calories: i32
}

fn load_elves_from_file(file_name: &str) -> Vec<Elf> {
    let file = File::open(file_name);
    let mut elves = Vec::new();
    match file {
        Result::Err(_) => Vec::new(),
        Result::Ok(mut file) => {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);
            let splits = contents.as_str().split("\n\n");
            for split in splits {
                let mut calories = 0;
                for calorie_count in split.split("\n") {
                    if let Result::Ok(calorie_i) = calorie_count.parse::<i32>() {
                        calories += calorie_i;
                    }
                }
                elves.push(Elf {
                    calories: calories
                });
            }
            elves
        }
    }
}

fn main() {
    let mut elves = load_elves_from_file("./input.txt");
    println!("{:?}", elves);
    elves.sort_by(|a, b| a.calories.cmp(&b.calories));
    println!("{:?}", elves.iter().rev().take(3).map(|a| a.calories).sum::<i32>())
}
