use std::cmp::Ord;

#[derive(Debug, Eq, Clone)]
struct Cell {
    n: i32,
    low: bool,
    visited: bool,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.n.cmp(&other.n)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Cell {
    fn new(n: i32) -> Self {
        Self {
            n,
            low: false,
            visited: false,
        }
    }
    fn is_low(&self) -> bool {
        self.low
    }
}

struct HeightMap(Vec<Vec<Cell>>);

impl HeightMap {
    fn new() -> Self {
        Self(vec![])
    }
    fn height_map(&self) -> &Vec<Vec<Cell>> {
        &self.0
    }
    fn row(&self) -> i32 {
        self.0.len() as i32
    }
    fn col(&self) -> i32 {
        if let Some(v) = self.0.get(0) {
            return v.len() as i32;
        }
        0
    }
    fn get(&self, i: i32, j: i32) -> Option<&Cell> {
        if i < 0 || j < 0 {
            return None;
        }
        if let Some(v) = self.0.get(i as usize) {
            return v.get(j as usize);
        }
        None
    }
    fn get_mut(&mut self, i: i32, j: i32) -> Option<&mut Cell> {
        if i < 0 || j < 0 {
            return None;
        }
        if let Some(v) = self.0.get_mut(i as usize) {
            return v.get_mut(j as usize);
        }
        None
    }
    fn push(&mut self, v: Vec<Cell>) {
        self.0.push(v)
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("--------------------");
        for v in &self.0 {
            for c in v {
                if c.is_low() {
                    print!("[")
                }
                print!("{}", c.n);
                if c.is_low() {
                    print!("]")
                }
            }
            println!();
        }
    }
}
fn main() {
    let input = include_str!("input");
    let mut map = HeightMap::new();
    for line in input.lines() {
        let mut line_num = vec![];
        for c in line.chars() {
            line_num.push(Cell::new(c as i32 - '0' as i32));
        }
        map.push(line_num);
    }
    low_point(&mut map);
    part1(&map);
    part2(&mut map);
}

fn dfs(pos: (i32, i32), map: &mut HeightMap) -> i32 {
    let mut cnt = 0;
    let cell = map.get_mut(pos.0, pos.1).unwrap();
    if cell.visited || cell.n == 9 {
        return 0;
    }
    cell.visited = true;

    let row = map.row();
    let col = map.col();
    cnt += 1;
    //up
    if pos.0 > 0 {
        cnt += dfs((pos.0 - 1, pos.1), map)
    }
    //down
    if pos.0 < row - 1 {
        cnt += dfs((pos.0 + 1, pos.1), map)
    }
    //left
    if pos.1 > 0 {
        cnt += dfs((pos.0, pos.1 - 1), map)
    }

    //right
    if pos.1 < col - 1 {
        cnt += dfs((pos.0, pos.1 + 1), map)
    }

    cnt
}

fn part2(map: &mut HeightMap) {
    let mut list = vec![];
    for i in 0..map.row() {
        for j in 0..map.col() {
            let cell = map.get(i, j).unwrap();
            if cell.is_low() {
                let cnt = dfs((i, j), map);
                list.push(cnt);
            }
        }
    }
    list.sort_unstable();
    list.reverse();
    let mut multi = 1;
    list.get(0..3).unwrap().iter().for_each(|n| multi *= n);
    println!("part 2: {:?}, multi {:?}", list.get(0..3), multi);
}

fn part1(map: &HeightMap) {
    let mut ans = 0;
    for v in map.height_map() {
        for cell in v {
            if cell.is_low() {
                ans += cell.n + 1;
            }
        }
    }
    println!("part 1: {}", ans);
}

fn low_point(map: &mut HeightMap) {
    let row = map.row();
    let col = map.col();
    for i in 0..row as i32 {
        for j in 0..col as i32 {
            let cell = map.get(i, j).unwrap();
            let mut low = true;
            // up
            if let Some(v) = map.get(i - 1, j) {
                if v <= cell {
                    low = false;
                }
            }
            // down
            if let Some(v) = map.get(i + 1, j) {
                if v <= cell {
                    low = false;
                }
            }
            // left
            if let Some(v) = map.get(i, j - 1) {
                if v <= cell {
                    low = false;
                }
            }
            // right
            if let Some(v) = map.get(i, j + 1) {
                if v <= cell {
                    low = false;
                }
            }
            if low {
                map.get_mut(i, j).unwrap().low = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cmp() {
        let cell1 = Cell::new(1);
        let cell2 = Cell::new(2);
        let cell3 = Cell::new(3);

        assert!(cell1 == cell1);
        assert!(cell1 < cell2);
        assert!(cell3 > cell2);
    }
}
