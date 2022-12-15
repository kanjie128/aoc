use std::{cell::RefCell, rc::Rc};

const DIR_MAX_SIZE: usize = 100000;
const DISK_TOTAL_SPACE: usize = 70000000;
const DISK_SPACE_NEED: usize = 30000000;
static INPUT: &str = include_str!("input");

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            files: vec![],
            dirs: vec![],
        }
    }

    fn parse_input(input: &str) -> Rc<RefCell<Self>> {
        let mut dir_stack = vec![];
        let root = "/";
        let dir = Rc::new(RefCell::new(Dir::new(root)));
        dir_stack.push(dir);
        for line in input.lines() {
            if line.starts_with('$') {
                if line.contains("cd") {
                    let dir_name = line.split(' ').last().unwrap();
                    if dir_name == ".." {
                        dir_stack.pop();
                    } else {
                        let mut cd_dir = None;
                        for dir in dir_stack.last().unwrap().borrow().dirs.iter() {
                            if dir.borrow().name == dir_name {
                                cd_dir = Some(Rc::clone(dir));
                                break;
                            }
                        }
                        if let Some(dir) = cd_dir {
                            dir_stack.push(dir);
                        }
                    }
                }
            } else if line.starts_with("dir") {
                let dir_name = line.split(' ').last().unwrap();
                let dir = Rc::new(RefCell::new(Dir::new(dir_name)));
                dir_stack.last().unwrap().borrow_mut().dirs.push(dir);
            } else {
                let mut file_line = line.split(' ');
                let file_size = file_line.next().unwrap().parse::<usize>().unwrap();
                let file_name = file_line.next().unwrap();
                let file = File::new(file_name.to_string(), file_size);
                dir_stack.last().unwrap().borrow_mut().files.push(file);
            }
        }
        dir_stack.first().unwrap().to_owned()
    }

    fn dir_size(&self, size_vec: &mut Vec<usize>) -> usize {
        let mut dir_size = self.files.iter().map(|file| file.size).sum();
        for dir in self.dirs.iter() {
            dir_size += dir.borrow().dir_size(size_vec);
        }
        size_vec.push(dir_size);
        dir_size
    }

    fn part1(size_vec: &[usize]) -> usize {
        size_vec
            .iter()
            .filter(|size| **size <= DIR_MAX_SIZE)
            .sum::<usize>()
    }

    fn part2(size_vec: &mut [usize]) -> usize {
        size_vec.sort();
        let used_space = size_vec.last().unwrap();
        let space_need = DISK_SPACE_NEED - (DISK_TOTAL_SPACE - used_space);
        match size_vec.binary_search(&space_need) {
            Ok(i) | Err(i) => size_vec[i],
        }
    }

    #[allow(dead_code)]
    fn dump(&self, depth: usize) {
        println!("{}- {} (dir)", "  ".repeat(depth), self.name);
        for file in self.files.iter() {
            println!(
                "{}- {} (file, size={})",
                "  ".repeat(depth + 1),
                file.name,
                file.size
            );
        }
        for dir in self.dirs.iter() {
            dir.borrow().dump(depth + 1);
        }
    }
}

fn main() {
    let dir = Dir::parse_input(INPUT);
    let mut size_vec = vec![];
    dir.borrow().dir_size(&mut size_vec);
    println!("part1: {}", Dir::part1(&size_vec));
    println!("part2: {}", Dir::part2(&mut size_vec));
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let dir = Dir::parse_input(SAMPLE);
        let mut size_vec = vec![];
        dir.borrow().dir_size(&mut size_vec);
        assert_eq!(95437_usize, Dir::part1(&size_vec))
    }
    #[test]
    fn test_part2_sample() {
        let dir = Dir::parse_input(SAMPLE);
        let mut size_vec = vec![];
        dir.borrow().dir_size(&mut size_vec);
        assert_eq!(24933642_usize, Dir::part2(&mut size_vec));
    }
}
