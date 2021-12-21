use std::collections::HashMap;
use std::collections::HashSet;

struct Graph<'a> {
    graph: Vec<Vec<i32>>,
    visited: HashSet<usize>,
    small_cave_visited: HashMap<usize, usize>,
    path_cnt1: usize,
    path_cnt2: usize,
    cave_index: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn new(cave_index: HashMap<&'a str, usize>) -> Self {
        Self {
            graph: vec![],
            visited: HashSet::new(),
            small_cave_visited: HashMap::new(),
            path_cnt1: 0,
            path_cnt2: 0,
            cave_index,
        }
    }
    fn build(&mut self, paths: &[Path]) {
        self.graph.reserve(self.cave_index.len());
        for _ in 0..self.cave_index.len() {
            self.graph.push([0].repeat(self.cave_index.len()));
        }
        for path in paths {
            let i = self.cave_index.get(path.start).unwrap();
            let j = self.cave_index.get(path.end).unwrap();
            *self.graph.get_mut(*i).unwrap().get_mut(*j).unwrap() = 1;
            *self.graph.get_mut(*j).unwrap().get_mut(*i).unwrap() = 1;
        }
    }
    fn dump(&self) {
        for row in &self.graph {
            for col in row {
                print!(" {} ", *col)
            }
            println!();
        }
    }
    fn get_pos(&self, pos: (usize, usize)) -> Option<&i32> {
        if let Some(v) = self.graph.get(pos.0) {
            return v.get(pos.1);
        }
        None
    }
    fn is_small(&self, idx: usize) -> bool {
        for (k, v) in &self.cave_index {
            if *v == idx && ('a'..='z').contains(&k.chars().next().unwrap()) {
                return true;
            }
        }
        false
    }
    fn is_small_cave_visited_twice(&self) -> bool {
        for v in self.small_cave_visited.values() {
            if *v >= 2 {
                return true;
            }
        }
        false
    }
    fn dfs(&mut self, start: usize, end: usize) {
        if start == end {
            self.path_cnt1 += 1;
            return;
        }
        for i in 0..self.graph.len() {
            if self.get_pos((start, i)) == Some(&1) && self.visited.get(&i).is_none() {
                if self.is_small(i) {
                    self.visited.insert(i);
                }
                self.dfs(i, end);
                self.visited.remove(&i);
            }
        }
    }

    fn dfs2(&mut self, start: usize, end: usize) {
        if start == end {
            self.path_cnt2 += 1;
            return;
        }
        for i in 0..self.graph.len() {
            if self.get_pos((start, i)) == Some(&1) && self.visited.get(&i).is_none() {
                if self.is_small(i) {
                    let twice = self.is_small_cave_visited_twice();
                    if let Some(n) = self.small_cave_visited.get_mut(&i) {
                        if twice {
                            continue;
                        }
                        *n += 1;
                        if *n >= 2 {
                            self.visited.insert(i);
                        }
                    } else {
                        self.small_cave_visited.insert(i, 1);
                        if twice {
                            self.visited.insert(i);
                        }
                    }
                }
                self.dfs2(i, end);
                self.visited.remove(&i);
                if let Some(n) = self.small_cave_visited.get_mut(&i) {
                    *n -= 1;
                    if *n == 0 {
                        self.small_cave_visited.remove(&i);
                    }
                }
            }
        }
    }
    fn get_cave_index(&self, k: &str) -> Option<usize> {
        if let Some(v) = self.cave_index.get(k) {
            return Some(*v);
        }
        None
    }
    fn part1(&mut self) {
        let start = self.get_cave_index("start").unwrap();
        let end = self.get_cave_index("end").unwrap();
        self.visited.clear();
        self.visited.insert(start);
        self.dfs(start, end);
        println!("part 1: {}", self.path_cnt1);
    }
    fn part2(&mut self) {
        let start = self.get_cave_index("start").unwrap();
        let end = self.get_cave_index("end").unwrap();
        self.visited.clear();
        self.visited.insert(start);
        self.dfs2(start, end);
        println!("part 2: {}", self.path_cnt2);
    }
}

struct Path<'a> {
    start: &'a str,
    end: &'a str,
}

fn main() {
    let s = include_str!("input");
    let mut paths = vec![];
    let mut cave_index = HashMap::<&str, usize>::new();
    for line in s.trim().lines() {
        let path = line.trim().split('-').collect::<Vec<_>>();
        assert_eq!(path.len(), 2);
        let start = path.get(0).unwrap();
        let end = path.get(1).unwrap();
        paths.push(Path { start, end });
        let idx = cave_index.len();
        if cave_index.get(start).is_none() {
            cave_index.insert(start, idx);
        }
        let idx = cave_index.len();
        if cave_index.get(end).is_none() {
            cave_index.insert(end, idx);
        }
    }
    let mut graph = Graph::new(cave_index);
    graph.build(&paths);
    graph.dump();
    graph.part1();
    graph.part2();
}
