use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn when_marker(stream: &str, window_size: usize) -> i32 {
    let slice = stream.chars().collect::<Vec<char>>();
    let windows = slice.windows(window_size);
    for (x, window) in windows.enumerate() {
        if !window
            .iter()
            .any(|x| window.iter().filter(|c| **c == *x).count() > 1)
        {
            return (x + window_size).try_into().unwrap();
        }
    }
    return 0;
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    println!("{:?}", when_marker(&line, 4));
    println!("{:?}", when_marker(&line, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(when_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(when_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(when_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(when_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(when_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(when_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(when_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(when_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(when_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
        assert_eq!(when_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26)
    }
}
