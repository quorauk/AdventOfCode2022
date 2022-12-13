use regex::Regex;
use std::{fs::File, io::Read, ops::Range};

fn find_ranges(string: &str) -> Option<(Range<i32>, Range<i32>)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let cap = re.captures(string);
    match cap {
        Some(cap) => Some((
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
                ..cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<i32>().unwrap()
                ..cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        )),
        None => None,
    }
}

fn range_encaps_another(x: &Option<(Range<i32>, Range<i32>)>) -> bool {
    let (r1, r2) = x.clone().unwrap();
    r1.end >= r2.end && r1.start <= r2.start || r2.end >= r1.end && r2.start <= r1.start
}

fn ranges_overlap(x: &Option<(Range<i32>, Range<i32>)>) -> bool {
    let (r1, r2) = x.clone().unwrap();
    if r2.start <= r1.start && r1.start <= r2.end {
        return true;
    }
    if r2.start <= r1.end && r1.end <= r2.end {
        return true;
    }
    r1.end >= r2.end && r1.start <= r2.start || r2.end >= r1.end && r2.start <= r1.start
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
            .map(|line| find_ranges(line))
            .filter(|x| x.is_some())
            .filter(|x| range_encaps_another(x))
            .count()
    );
    println!(
        "{}",
        contents
            .split("\n")
            .map(|line| find_ranges(line))
            .filter(|x| x.is_some())
            .filter(|x| ranges_overlap(x))
            .count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps() {
        let r1 = Some((5..7, 7..9));
        let r2 = Some((2..3, 4..5));
        let r3 = Some((5..7, 7..9));
        let r4 = Some((2..8, 3..7));
        let r5 = Some((6..6, 4..6));
        let r6 = Some((2..4, 4..8));
        assert_eq!(ranges_overlap(&r1), true);
        assert_eq!(ranges_overlap(&r2), false);
        assert_eq!(ranges_overlap(&r3), true);
        assert_eq!(ranges_overlap(&r4), true);
        assert_eq!(ranges_overlap(&r5), true);
        assert_eq!(ranges_overlap(&r6), true)
    }
}
