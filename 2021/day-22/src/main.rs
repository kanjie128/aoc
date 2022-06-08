use core::panic;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum CubeState {
    On,
    Off,
}

impl From<&str> for CubeState {
    fn from(s: &str) -> Self {
        if s == "on" {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl From<CubeState> for i64 {
    fn from(c: CubeState) -> Self {
        match c {
            CubeState::Off => 0,
            CubeState::On => 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair(i64, i64);

#[derive(Debug, Copy, Clone)]
struct Cube {
    state: CubeState,
    x: Pair,
    y: Pair,
    z: Pair,
}

impl Cube {
    fn new(state: CubeState, x: Pair, y: Pair, z: Pair) -> Self {
        Self { state, x, y, z }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{:?}|{:?}|{:?}|{:?}", self.state, self.x, self.y, self.z);
    }
}

struct XYZRange {
    xs: Vec<i64>,
    ys: Vec<i64>,
    zs: Vec<i64>,
    cube_grid: Vec<Vec<Vec<i64>>>,
}

impl XYZRange {
    fn new() -> Self {
        Self {
            xs: vec![],
            ys: vec![],
            zs: vec![],
            cube_grid: vec![],
        }
    }
    fn sort(&mut self) {
        self.xs.sort_unstable();
        self.ys.sort_unstable();
        self.zs.sort_unstable();
    }

    fn add_xs(&mut self, x: Pair) {
        self.xs.push(x.0);
        self.xs.push(x.1);
    }
    fn add_ys(&mut self, y: Pair) {
        self.ys.push(y.0);
        self.ys.push(y.1);
    }
    fn add_zs(&mut self, z: Pair) {
        self.zs.push(z.0);
        self.zs.push(z.1);
    }

    fn build_cube_grid(&mut self, cubes: &[Cube]) {
        let n = self.xs.len();
        self.cube_grid = vec![vec![vec![0i64; n]; n]; n];
        for cube in cubes {
            let x0 = self.xs.binary_search(&cube.x.0).unwrap();
            let x1 = self.xs.binary_search(&cube.x.1).unwrap();
            let y0 = self.ys.binary_search(&cube.y.0).unwrap();
            let y1 = self.ys.binary_search(&cube.y.1).unwrap();
            let z0 = self.zs.binary_search(&cube.z.0).unwrap();
            let z1 = self.zs.binary_search(&cube.z.1).unwrap();
            for x in x0..x1 {
                for y in y0..y1 {
                    for z in z0..z1 {
                        self.cube_grid[x][y][z] = cube.state.into();
                    }
                }
            }
        }
    }

    fn count_cube_on(&self) -> i64 {
        let mut sum = 0;
        for (i, x) in self.cube_grid.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                for (k, z) in y.iter().enumerate() {
                    if *z == 1 {
                        let x = self.xs[i + 1] - self.xs[i];
                        let y = self.ys[j + 1] - self.ys[j];
                        let z = self.zs[k + 1] - self.zs[k];
                        sum += x * y * z;
                    }
                }
            }
        }
        sum
    }
}
fn main() {
    let s = include_str!("input");
    let reg =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    let mut cubes = vec![];
    let mut xyz_range = XYZRange::new();
    for line in s.lines() {
        for cap in reg.captures_iter(line) {
            if cap.len() != 8 {
                eprintln!("invalid line: {}", line);
                panic!("invalid");
            }
            let xyz = cap
                .iter()
                .filter_map(|p| p.and_then(|s| s.as_str().parse::<i64>().ok()))
                .collect::<Vec<_>>();
            let x = Pair(xyz[0], xyz[1] + 1);
            let y = Pair(xyz[2], xyz[3] + 1);
            let z = Pair(xyz[4], xyz[5] + 1);
            let cube = Cube::new(cap[1].into(), x, y, z);
            cubes.push(cube);
            xyz_range.add_xs(x);
            xyz_range.add_ys(y);
            xyz_range.add_zs(z);
        }
    }
    // cubes.iter().for_each(|cube| cube.print());
    let filter = |p: Pair| (-50..=50).contains(&p.0) && (-50..=50).contains(&p.1);
    xyz_range.sort();
    let cubes_part1 = cubes
        .iter()
        .filter(|cube| filter(cube.x) && filter(cube.y) && filter(cube.z))
        .copied()
        .collect::<Vec<_>>();
    xyz_range.build_cube_grid(&cubes_part1);
    println!("part1: {}", xyz_range.count_cube_on());
    xyz_range.build_cube_grid(&cubes);
    println!("part2: {}", xyz_range.count_cube_on());
}
