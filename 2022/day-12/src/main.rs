use std::{cell::Cell, ops::Add};

static INPUT: &str = include_str!("input");
#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: i32,
    col: i32,
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl Point {
    fn valid(self, max_row: i32, max_col: i32) -> bool {
        self.row >= 0 && self.row < max_row && self.col >= 0 && self.col < max_col
    }
    fn neighbor(self, max_row: i32, max_col: i32) -> Vec<Self> {
        let mut nb = vec![];
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for dir in dirs {
            let p = self + dir;
            if Self::valid(p, max_row, max_col) {
                nb.push(p);
            }
        }
        nb
    }
}

#[derive(Default, Clone)]
struct Elevation {
    value: char,
    visited: Cell<i32>,
}
impl Elevation {
    fn new(c: char) -> Self {
        Self {
            value: c,
            visited: Cell::new(-1),
        }
    }
}
#[derive(Default, Clone)]
struct HeightMap {
    elevations: Vec<Vec<Elevation>>,
    start: Point,
    end: Point,
}

impl HeightMap {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn parse_input(input: &str) -> Self {
        let mut hight_map = Self::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, mut c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (i, j);
                    c = 'a';
                } else if c == 'E' {
                    end = (i, j);
                    c = 'z';
                }
                row.push(Elevation::new(c));
            }
            hight_map.elevations.push(row);
        }
        hight_map.start = Point {
            row: start.0 as i32,
            col: start.1 as i32,
        };
        hight_map.end = Point {
            row: end.0 as i32,
            col: end.1 as i32,
        };
        hight_map
    }

    fn get_elevation(&self, p: Point) -> &Elevation {
        self.elevations
            .get(p.row as usize)
            .unwrap()
            .get(p.col as usize)
            .unwrap()
    }

    fn get_elevation_visited(&self, p: Point) -> i32 {
        self.get_elevation(p).visited.get()
    }

    fn set_elevation_visited(&self, p: Point, visited: i32) {
        self.elevations
            .get(p.row as usize)
            .unwrap()
            .get(p.col as usize)
            .unwrap()
            .visited
            .set(visited);
    }

    fn fewest_path(&self, start: Point) -> i32 {
        let max_row = self.elevations.len();
        let max_col = self.elevations.get(0).unwrap().len();
        self.set_elevation_visited(start, 0);
        let mut search_path = vec![start];

        let point_can_visit = |p: Point, n: Point| -> bool {
            self.get_elevation_visited(n) < 0
                && (self.get_elevation(n).value as i32 - self.get_elevation(p).value as i32 <= 1)
        };

        while !search_path.is_empty() {
            let mut new_search = vec![];
            for p in search_path {
                for n in p.neighbor(max_row as i32, max_col as i32) {
                    if point_can_visit(p, n) {
                        if n == self.end {
                            return self.get_elevation_visited(p) + 1;
                        }
                        self.set_elevation_visited(n, self.get_elevation_visited(p) + 1);
                        new_search.push(n);
                    }
                }
            }
            search_path = new_search;
        }
        0
    }
    fn fewest_path2(&self) -> i32 {
        let mut a_start = vec![];
        for (i, row) in self.elevations.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col.value == 'a' {
                    a_start.push(Point {
                        row: i as i32,
                        col: j as i32,
                    });
                }
            }
        }

        let mut fewest = vec![];
        for a in a_start {
            let hm = self.clone();
            fewest.push(hm.fewest_path(a));
        }
        *fewest.iter().filter(|n| **n > 0).min().unwrap()
    }
}

fn main() {
    let hight_map = HeightMap::parse_input(INPUT);
    println!("part1: {:?}", hight_map.fewest_path(hight_map.start));
    let hight_map = HeightMap::parse_input(INPUT);
    println!("part2: {:?}", hight_map.fewest_path2());
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let hight_map = HeightMap::parse_input(SAMPLE);
        assert_eq!(hight_map.fewest_path(hight_map.start), 31);
    }
    #[test]
    fn test_part2_sample() {
        let hight_map = HeightMap::parse_input(SAMPLE);
        assert_eq!(hight_map.fewest_path2(), 29);
    }
}
