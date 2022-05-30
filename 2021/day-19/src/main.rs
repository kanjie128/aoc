use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct Distance(i32, i32, i32);

impl Distance {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self(x, y, z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("coordinate must has (x, y, z)")]
    CoordinateMissing,
}

impl Beacon {
    fn parse(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let xyz = line
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        if xyz.len() != 3 {
            return Err(Box::new(ParseError::CoordinateMissing));
        }
        Ok(Self {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        })
    }

    fn distance(s1: &Beacon, s2: &Beacon) -> Distance {
        Distance::new(s1.x - s2.x, s1.y - s2.y, s1.z - s2.z)
    }

    fn shift(&self, v: &[i32]) -> Self {
        Self {
            x: self.x * v[0],
            y: self.y * v[1],
            z: self.z * v[2],
        }
    }

    fn sub(&mut self, distance: Distance) {
        self.x -= distance.0;
        self.y -= distance.1;
        self.z -= distance.2;
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    index: usize,
    position: (i32, i32, i32),
    beacons: Vec<Beacon>,
}

impl std::cmp::PartialEq for Scanner {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Scanner {
    fn new(index: usize) -> Self {
        Self {
            index,
            position: (0, 0, 0),
            beacons: vec![],
        }
    }

    fn manhattan_distance(s1: &Scanner, s2: &Scanner) -> usize {
        ((s1.position.0 - s2.position.0).abs()
            + (s1.position.1 - s2.position.1).abs()
            + (s1.position.2 - s2.position.2).abs()) as usize
    }

    fn match_beacons(&mut self, distance: Distance, s2: &Scanner) -> i32 {
        let mut cnt = 0;
        for b in self.beacons.iter_mut() {
            b.sub(distance);
            if s2.beacons.contains(&(*b)) {
                cnt += 1;
            }
        }
        cnt
    }

    fn axis_shift(p: &[i32], axis: (usize, usize, usize)) -> Beacon {
        Beacon {
            x: p[axis.0 - 1],
            y: p[axis.1 - 1],
            z: p[axis.2 - 1],
        }
    }

    fn pattern(s: &Scanner, axis: (usize, usize, usize), pattern: &Vec<Vec<i32>>) -> Vec<Scanner> {
        let mut scanners = vec![];
        for p in pattern {
            let mut scanner = Scanner::new(s.index);
            for b in &s.beacons {
                let b = Self::axis_shift(&[b.x, b.y, b.z], axis);
                scanner.beacons.push(b.shift(p));
            }
            scanners.push(scanner);
        }
        scanners
    }

    fn rotate(&self) -> Vec<Scanner> {
        let pattern1 = vec![
            vec![1, 1, 1],
            vec![1, -1, -1],
            vec![-1, 1, -1],
            vec![-1, -1, 1],
        ];
        let pattern2 = vec![
            vec![1, 1, -1],
            vec![1, -1, 1],
            vec![-1, 1, 1],
            vec![-1, -1, -1],
        ];
        let mut scanners = vec![];
        scanners.append(&mut Self::pattern(self, (1, 2, 3), &pattern1));
        scanners.append(&mut Self::pattern(self, (2, 3, 1), &pattern1));
        scanners.append(&mut Self::pattern(self, (3, 1, 2), &pattern1));
        scanners.append(&mut Self::pattern(self, (1, 3, 2), &pattern2));
        scanners.append(&mut Self::pattern(self, (2, 1, 3), &pattern2));
        scanners.append(&mut Self::pattern(self, (3, 2, 1), &pattern2));
        scanners
    }

    fn common_beacons(s1: &Scanner, s2: &Scanner) -> (i32, Distance) {
        let mut cnt = 0;
        for b1 in &s1.beacons {
            for b2 in &s2.beacons {
                let distance = Beacon::distance(b1, b2);
                let mut s = s1.clone();
                cnt = std::cmp::max(cnt, s.match_beacons(distance, s2));
                if cnt >= 12 {
                    return (cnt, distance);
                }
            }
        }
        (0, Distance::new(0, 0, 0))
    }
}

fn main() {
    let s = include_str!("input");
    let mut scanners = vec![];
    let mut scanner = Scanner::new(0);
    let mut cnt = 0;
    for line in s.lines() {
        if line.contains("---") {
            continue;
        }
        if line.is_empty() {
            scanners.push(scanner);
            cnt += 1;
            scanner = Scanner::new(cnt);
        } else {
            let res = Beacon::parse(line);
            match res {
                Ok(b) => scanner.beacons.push(b),
                Err(e) => {
                    eprintln!("parse {} error: {}", line, e);
                    return;
                }
            }
        }
    }
    scanners.push(scanner);

    let mut scanner_queue = vec![scanners[0].clone()];
    let mut visited = vec![];

    loop {
        if scanner_queue.len() == scanners.len() {
            break;
        }

        let mut new_scanner = vec![];
        for s in &scanner_queue {
            if scanner_visited(&visited, s.index) {
                continue;
            }
            for s2 in &scanners {
                if s2.index == s.index || scanner_visited(&visited, s2.index) {
                    continue;
                }
                let mut rotate_scanners = s2.rotate();
                for rs in &mut rotate_scanners {
                    let (comm, distance) = Scanner::common_beacons(s, rs);
                    if comm >= 12 {
                        println!(
                            "scanner {} -> {}, common beacons: {comm}, distance: {distance:?}",
                            s.index, rs.index,
                        );
                        if !scanner_visited(&visited, rs.index) && !new_scanner.contains(rs) {
                            rs.position = (
                                distance.0 + s.position.0,
                                distance.1 + s.position.1,
                                distance.2 + s.position.2,
                            );
                            new_scanner.push(rs.clone());
                        }
                        break;
                    }
                }
            }
            visited.push(s.index);
        }
        for s in new_scanner {
            scanner_queue.push(s);
        }
    }
    let mut set = HashSet::new();
    for s in &scanner_queue {
        for b in &s.beacons {
            set.insert(Beacon {
                x: b.x + s.position.0,
                y: b.y + s.position.1,
                z: b.z + s.position.2,
            });
        }
    }
    println!("part1: {}", set.len());
    let mut manhattan_distance = 0;
    for (i, s1) in scanner_queue.iter().enumerate() {
        for (j, s2) in scanner_queue.iter().enumerate() {
            if j > i {
                manhattan_distance =
                    std::cmp::max(manhattan_distance, Scanner::manhattan_distance(s1, s2));
            }
        }
    }
    println!("part2: {}", manhattan_distance);
}

fn scanner_visited(visited: &Vec<usize>, scanner: usize) -> bool {
    for index in visited {
        if *index == scanner {
            return true;
        }
    }
    false
}
