use regex::Regex;
use std::collections::HashMap;

use std::io::Read;

use data_structures::tree::Tree;
use std::fs;

trait FileSystem {
    fn ls(&mut self, lines: &str);
    fn cd(&mut self, name: String);
    fn dir_sizes(&self) -> HashMap<usize, i32>;
}

impl FileSystem for Tree<FileSystemEntry> {
    fn ls(&mut self, lines: &str) {
        let re_dir = Regex::new(r"^dir (?P<name>.*)$").unwrap();
        let re_file = Regex::new(r"^(?P<size>\d+) (?P<name>.*)$").unwrap();
        for line in lines.split('\n') {
            if re_dir.is_match(line) {
                let captures = re_dir.captures(line).unwrap();
                self.push(FileSystemEntry::Dir(Directory {
                    name: captures.name("name").unwrap().as_str().to_string(),
                }))
            } else if re_file.is_match(line) {
                let captures = re_file.captures(line).unwrap();
                self.push(FileSystemEntry::File(File {
                    name: captures.name("name").unwrap().as_str().to_string(),
                    size: captures.name("size").unwrap().as_str().parse().unwrap(),
                }))
            }
        }
    }

    fn cd(&mut self, name: String) {
        if name == "/" {
            self.change_target(0)
        } else if name == ".." {
            let current = self.nodes.get(self.current_node).unwrap();
            self.change_target(current.parent.unwrap_or(0))
        } else {
            for (i, node) in self.nodes.iter().enumerate() {
                if node.parent == Some(self.current_node) {
                    let e_name = match &node.elem {
                        FileSystemEntry::Dir(d) => d.name.clone(),
                        FileSystemEntry::File(f) => f.name.clone(),
                    };

                    if name == e_name {
                        self.current_node = i;
                        return;
                    }
                }
            }
        }
    }

    fn dir_sizes(&self) -> HashMap<usize, i32> {
        let mut sizes = HashMap::new();
        let nodes_clone = self.nodes.clone();
        for node in nodes_clone.iter() {
            match &node.elem {
                FileSystemEntry::File(d) => {
                    let mut cur = node.parent;
                    while let Some(index) = cur {
                        *sizes.entry(index).or_insert(0) += d.size;
                        let new_node = self.nodes.get(index);
                        if let Some(node) = new_node {
                            cur = node.parent;
                        } else {
                            cur = None;
                        }
                    }
                }
                _ => (),
            }
        }
        sizes
    }
}

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    name: String,
}

#[derive(Debug, PartialEq, Clone)]
enum FileSystemEntry {
    Dir(Directory),
    File(File),
}

fn build_tree(contents: String) -> Tree<FileSystemEntry> {
    let cmd_re = Regex::new(r"\n?\$ ").unwrap();
    let cmds = cmd_re.split(contents.as_str());
    let mut tree = Tree::new(FileSystemEntry::Dir(Directory {
        name: "/".to_string(),
    }));
    let ls_re = Regex::new(r"ls\n(?P<rest>(.|\n)*)").unwrap();
    let cd_re = Regex::new(r"cd (?P<name>.*)").unwrap();
    for cmd in cmds {
        if ls_re.is_match(cmd) {
            let caps = ls_re.captures(cmd).unwrap();
            let rest = caps.name("rest").unwrap().as_str();
            tree.ls(rest)
        } else if cd_re.is_match(cmd) {
            let caps = cd_re.captures(cmd).unwrap();
            let name = caps.name("name").unwrap().as_str();
            tree.cd(name.to_string())
        }
    }
    tree
}

fn main() {
    let mut file = fs::File::open("./a07/input.txt").unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let tree = build_tree(contents);
    let sum: i32 = tree
        .dir_sizes()
        .into_values()
        .into_iter()
        .filter(|y| *y < 100000)
        .map(|y| y.clone())
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("part 1: {:?}", sum);
    let total_size: i32 = tree.dir_sizes().get(&0).unwrap().clone();
    let unused_space = 70000000 - total_size;
    let to_delete = 30000000 - unused_space;
    println!("unused: {:?}", total_size);
    println!("to_delete: {:?}", to_delete);
    let smallest_viable = tree
        .dir_sizes()
        .into_values()
        .into_iter()
        .filter(|x| x > &to_delete)
        .min();
    println!("part 2: {:?}", smallest_viable);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_ls() {
        let mut tree = Tree::new(FileSystemEntry::Dir(Directory {
            name: "/".to_string(),
        }));
        tree.ls("dir a\n1000 b.txt\n1000 c.dat");
        assert_eq!(tree.nodes.len(), 4)
    }

    #[test]
    fn test_cd() {
        let mut tree = Tree::new(FileSystemEntry::Dir(Directory {
            name: "/".to_string(),
        }));
        tree.ls("dir a\n1000 b.txt\n1000 c.dat");
        assert_eq!(tree.nodes.len(), 4);
        tree.cd("b.txt".to_string());
        assert_eq!(tree.current_node, 2)
    }

    #[test]
    fn test_test_txt() {
        let mut file = fs::File::open("./test.txt").unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let tree = build_tree(contents);
        assert_eq!(tree.nodes.len(), 14);
        println!("{:?}", tree.dir_sizes())
    }
}
