use std::{fs::File, io::Read};

fn read_trees(string: String) -> Vec<Vec<i8>> {
    let mut trees = Vec::new();
    for line in string.split('\n') {
        let mut tree_line = Vec::new();
        for tree in line.split("") {
            if let Ok(x) = tree.parse::<i8>() {
                tree_line.push(x);
            }
        }
        trees.push(tree_line)
    };
    trees
}


fn flip_horizontal<T>(grid: &mut Vec<Vec<T>>) {
    for row in grid {
        row.reverse()
    }
}

fn transpose<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    (0..grid[0].len())
    .map(|i| grid.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
    .collect()
}

fn look_in_dir(trees: &Vec<Vec<i8>>, visible: &mut Vec<Vec<bool>>) {
    for (row, line)in trees.iter().enumerate() {
        let mut current_max : i8 = -1;
        for (col, tree) in line.iter().enumerate() {
            if tree > &current_max {
                current_max = tree.clone();
                visible[row][col] = true;
            }
        }
    }
}

struct Grid {
    grid: Vec<Vec<i8>>
}

impl Grid {
    fn up(&self, x: usize, y: usize) -> Vec<i8> {
        if x > 0 {
            (0..x).rev().map(|x| self.grid[x][y]).collect()
        } else {
            vec![]
        }
    }

    fn down(&self, x: usize, y: usize) -> Vec<i8> {
        (x+1..self.grid.len()).map(|x| self.grid[x][y]).collect()
    }

    fn left(&self, x: usize, y: usize) -> Vec<i8> {
        if y > 0 {
            (0..y).rev().map(|y| self.grid[x][y]).collect()
        } else {
            vec![]
        }
    }

    fn right(&self, x: usize, y: usize) -> Vec<i8> {
        (y+1..self.grid[x].len()).map(|y| self.grid[x][y]).collect()
    }
}

fn senic_score_for(dir: Vec<i8>, tree: &i8) -> i32 {
    for i in dir.iter().enumerate() {
        if i.1 >= tree {
            return (i.0 + 1) as i32
        }
    }
    dir.len() as i32
}

fn scenic(string: String) -> i32 {
    let mut max_scenic = -1;
    let trees = read_trees(string);
    let grid = Grid {
        grid: trees
    };
    let rows = grid.grid.len();
    let cols = grid.grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            let tree = grid.grid[i][j];
            let left = senic_score_for(grid.left(i, j), &tree);
            let right = senic_score_for(grid.right(i, j), &tree);
            let up = senic_score_for(grid.up(i, j), &tree);
            let down = senic_score_for(grid.down(i, j), &tree);
            let sum = left * right * up * down;
            if sum > max_scenic {
                max_scenic = sum;
            };
        }
    }

    max_scenic
}

fn visible(string: String) -> usize {
    let mut trees = read_trees(string);
    let mut visible_trees: Vec<Vec<bool>> = trees.iter().map(|r| r.clone().iter().map(|_| false).collect()).collect();

    look_in_dir(&trees, &mut visible_trees);
    flip_horizontal(&mut trees);
    flip_horizontal(&mut visible_trees);
    look_in_dir(&trees, &mut visible_trees);
    trees = transpose(trees.clone());
    visible_trees = transpose(visible_trees.clone());
    look_in_dir(&trees, &mut visible_trees);
    flip_horizontal(&mut trees);
    flip_horizontal(&mut visible_trees);
    look_in_dir(&trees, &mut visible_trees);
    // trees = transpose(trees.clone());
    visible_trees = transpose(visible_trees.clone());

    visible_trees.iter().map(|r| r.iter().filter(|x| **x == true).map(|x| x.clone()).count()).sum()
}

fn main() {
    let file = File::open("./input.txt");
    let mut contents = String::new();
    let _ = file.unwrap().read_to_string(&mut contents);
    println!("pt1 : {:?}", visible(contents));
    let file = File::open("./input.txt");
    let mut contents = String::new();
    let _ = file.unwrap().read_to_string(&mut contents);
    println!("pt2 : {:?}", scenic(contents));
}

#[cfg(test)]
mod tests {
  use std::{fs::{File}, io::Read};
    use super::*;

    #[test]
    fn test_visible() {
        let file = File::open("./test.txt");
        assert!(file.is_ok());
        let mut contents = String::new();
        let _ = file.unwrap().read_to_string(&mut contents);
        assert_eq!(
            visible(contents),
            21
        );
        let file = File::open("./test copy.txt");
        assert!(file.is_ok());
        let mut contents = String::new();
        let _ = file.unwrap().read_to_string(&mut contents);
        assert_eq!(
            visible(contents),
            6
        )
    }

    #[test]
    fn test_scenic() {
        let file = File::open("./test.txt");
        assert!(file.is_ok());
        let mut contents = String::new();
        let _ = file.unwrap().read_to_string(&mut contents);
        assert_eq!(
            scenic(contents),
            8
        );
    }

    #[test]
    fn test_read_trees() {
        let x = String::from("159\n139\n104");
        assert_eq!(
            read_trees(x),
            vec![
                vec![1,5,9],
                vec![1,3,9],
                vec![1,0,4]
            ]
        );
    }
}