#![feature(test)]

extern crate test;
use std::{collections::{VecDeque, HashSet}};

#[derive(Debug)]
struct Maze {
    cells: Vec<Vec<char>>,
}

impl Maze {
    fn new(filename: &str) -> Self {
        let input = std::fs::read_to_string(filename).unwrap();
        let cells = input.lines().map(|l| l.chars().collect()).collect();
        Maze {
            cells,
        }
    }

    pub fn find_elem(&self, elem: char) -> Vec<(usize, usize)> {
        let mut elems = vec![];
        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                if self.cells[i][j] == elem {
                    elems.push((i, j))
                }
            }
        }
        elems
    }

    fn can_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (f_x, f_y) = from;
        let from_cell = self.cells[f_x][f_y];
        let (t_x, t_y) = to;
        let to_cell = self.cells[t_x][t_y];
        if from_cell == 'S' && to_cell == 'a' {
            return true
        }
        if from_cell == 'z' && to_cell == 'E' {
            return true
        } else if to_cell == 'E'{
            return false
        }
        (from_cell as u8) + 1 >= (to_cell as u8)
    }

    fn movement_options(&self, location: (usize, usize)) ->  Vec<(usize, usize)> {
        let (x, y) = location;
        let mut options = vec![];
        if x > 0 && self.can_move(location, (x-1, y)) {
            options.push((x-1, y));
        }
        if y > 0 && self.can_move(location, (x, y-1)) {
            options.push((x, y-1));
        }
        if x < &self.cells.len() - 1 && self.can_move(location, (x+1, y)){
            options.push((x+1, y));
        }
        if y < &self.cells[0].len() - 1 && self.can_move(location, (x, y+1)) {
            options.push((x, y+1));
        }
        options
    }

    fn navigate_maze(&self, start_points: &Vec<(usize, usize)>) -> i32 {
        let with_distances : Vec<((usize, usize), i32)> = start_points.iter().map(|x| (x.to_owned(), 0)).collect();
        let mut options: VecDeque<((usize, usize), i32)> = VecDeque::from(with_distances);
        let mut seen = HashSet::new();
        while let Some((top, distance)) = options.pop_back() {
            if self.cells[top.0][top.1] == 'E' {
                return distance
            }
            for option in self.movement_options(top) {
                if !seen.contains(&option) {
                    seen.insert(option);
                    options.push_front((option, distance + 1));

                }
            }
        }
        return 0
    }
}



fn main() {
    let maze = Maze::new("input.txt");
    let mut start = maze.find_elem('S');
    println!("pt1: {}", maze.navigate_maze(&start));
    start.extend(maze.find_elem('a'));
    println!("pt2: {}", maze.navigate_maze(&start));
}


#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    let maze = Maze::new("test.txt");
    let mut start = maze.find_elem('S');
    assert_eq!(
        maze.navigate_maze(&start),
        31
    );
    start.extend(maze.find_elem('a'));
    assert_eq!(
        maze.navigate_maze(&start),
        29
    )
  }

  #[bench]
  fn bench_loading(b: &mut Bencher) {
    let maze = Maze::new("test.txt");
    let mut start = maze.find_elem('S');
    b.iter(||
        maze.navigate_maze(&start)
    );
  }
}