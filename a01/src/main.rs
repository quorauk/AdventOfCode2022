#![feature(test)]

extern crate test;

fn new(filename: &str) -> (i32, i32) {
    let mut elves = get_elves(filename);
    elves.sort_by(|a, b| b.cmp(a));
    (elves[0], elves.iter().take(3).sum())
}

fn main() {
    let (p1, p2) = new("input.txt");
    println!("pt1: {:?}", p1);
    println!("pt2: {:?}", p2)
}

fn get_elves(filename: &str) -> Vec<i32> {
    let input = std::fs::read_to_string(filename).unwrap();
    input
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|x| x.parse::<i32>().unwrap_or(0)).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(new("input.txt"), (69289, 205615));
    }

    #[bench]
    fn bench_loading(b: &mut Bencher) {
        b.iter(|| get_elves("input.txt"))
    }

    #[bench]
    fn bench_all(b: &mut Bencher) {
        b.iter(|| new("input.txt"))
    }
}
