enum Direction {
    UP,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Clone, Copy)]
struct Pos(i32, i32);
impl Pos {
    fn move_step(&self, dir: Direction) -> Self {
        match dir {
            Direction::UP => Self(self.0 - 1, self.1),
            Direction::Down => Self(self.0 + 1, self.1),
            Direction::Left => Self(self.0, self.1 - 1),
            Direction::Right => Self(self.0, self.1 + 1),
            Direction::UpLeft => Self(self.0 - 1, self.1 - 1),
            Direction::UpRight => Self(self.0 - 1, self.1 + 1),
            Direction::DownLeft => Self(self.0 + 1, self.1 - 1),
            Direction::DownRight => Self(self.0 + 1, self.1 + 1),
        }
    }
}
#[derive(Debug)]
struct Octopus {
    energy: i32,
    flashed: bool,
}

impl Octopus {
    fn new(energy: i32) -> Self {
        Self {
            energy,
            flashed: false,
        }
    }
}

type EnergyMap = Vec<Vec<Octopus>>;

struct OctopusMap {
    map: EnergyMap,
    total_flash_cnt: i32,
    one_step_flash_cnt: i32,
    step_cnt: i32,
}

impl OctopusMap {
    fn new() -> Self {
        Self {
            map: vec![],
            total_flash_cnt: 0,
            one_step_flash_cnt: 0,
            step_cnt: 0,
        }
    }
    fn get(&self, pos: Pos) -> Option<&Octopus> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        if let Some(row) = self.map.get(pos.0 as usize) {
            return row.get(pos.1 as usize);
        }
        None
    }
    fn get_mut(&mut self, pos: Pos) -> Option<&mut Octopus> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        if let Some(row) = self.map.get_mut(pos.0 as usize) {
            return row.get_mut(pos.1 as usize);
        }
        None
    }

    fn row_len(&self) -> usize {
        self.map.len()
    }
    fn col_len(&self) -> usize {
        if let Some(row) = self.map.get(0) {
            return row.len();
        }
        0
    }
    fn push(&mut self, v: Vec<Octopus>) {
        self.map.push(v);
    }
    #[allow(dead_code)]
    fn dump(&self) {
        println!("--------------");
        for row in &self.map {
            for col in row {
                if col.flashed {
                    print!("[{}]", col.energy);
                } else {
                    print!(" {} ", col.energy);
                }
            }
            println!();
        }
        println!("flashed count: {}", self.one_step_flash_cnt);
    }
    fn flash_once(&mut self) {
        self.total_flash_cnt += 1;
        self.one_step_flash_cnt += 1;
    }
    fn is_all_flash(&self) -> bool {
        if self.one_step_flash_cnt as usize == self.col_len() * self.row_len() {
            return true;
        }
        false
    }
    fn step(&mut self) {
        self.step_cnt += 1;
        for row in 0..self.row_len() {
            for col in 0..self.col_len() {
                let pos = Pos(row as i32, col as i32);
                let octopus = self.get_mut(pos).unwrap();
                if !octopus.flashed {
                    octopus.energy += 1;
                    if octopus.energy > 9 {
                        self.flash(pos);
                    }
                }
            }
        }
    }
    fn clean_step(&mut self) {
        self.one_step_flash_cnt = 0;
        for row in 0..self.row_len() {
            for col in 0..self.col_len() {
                self.get_mut(Pos(row as i32, col as i32)).unwrap().flashed = false;
            }
        }
    }
    fn flash(&mut self, pos: Pos) {
        if self.get(pos).unwrap().flashed {
            return;
        }
        self.flash_once();
        self.get_mut(pos).unwrap().energy = 0;
        self.get_mut(pos).unwrap().flashed = true;

        self.energy_spread(pos.move_step(Direction::UP));
        self.energy_spread(pos.move_step(Direction::Down));
        self.energy_spread(pos.move_step(Direction::Right));
        self.energy_spread(pos.move_step(Direction::Left));
        self.energy_spread(pos.move_step(Direction::UpLeft));
        self.energy_spread(pos.move_step(Direction::UpRight));
        self.energy_spread(pos.move_step(Direction::DownRight));
        self.energy_spread(pos.move_step(Direction::DownLeft));
    }

    fn energy_spread(&mut self, pos: Pos) {
        if let Some(v) = self.get_mut(pos) {
            if !v.flashed {
                v.energy += 1;
                if v.energy > 9 {
                    self.flash(pos);
                }
            }
        }
    }
}
fn main() {
    let s = include_str!("input");

    let mut map = OctopusMap::new();
    for line in s.trim().lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Octopus::new(c as i32 - '0' as i32));
        }
        map.push(row);
    }
    let mut stop_1 = false;
    let mut stop_2 = false;
    loop {
        // map.dump();
        map.step();
        // map.dump();
        if map.step_cnt == 100 {
            stop_1 = true;
            println!("part 1: {}", map.total_flash_cnt);
        }
        if map.is_all_flash() {
            stop_2 = true;
            println!("part 2: {}", map.step_cnt);
        }
        if stop_1 && stop_2 {
            break;
        }
        map.clean_step();
    }
}
