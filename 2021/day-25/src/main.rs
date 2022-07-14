#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    East,
    South,
}
#[derive(Debug, Clone, Copy)]
struct Cell {
    state: State,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.state {
            State::Empty => ".",
            State::East => ">",
            State::South => "v",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct Land {
    move_steps_cnt: usize,
    cells: Vec<Vec<Cell>>,
}

impl Land {
    fn new() -> Self {
        Self {
            move_steps_cnt: 0,
            cells: vec![],
        }
    }

    fn move_east(&mut self) -> bool {
        let col_len = self.cells[0].len();
        let mut move_steps = vec![];
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let right = (j + 1) % col_len;
                if cell.state == State::East && self.cells[i][right].state == State::Empty {
                    move_steps.push((i, j, State::Empty));
                    move_steps.push((i, right, State::East));
                }
            }
        }

        for (i, j, state) in &move_steps {
            self.cells[*i][*j].state = *state
        }
        !move_steps.is_empty()
    }

    fn move_south(&mut self) -> bool {
        let row_len = self.cells.len();
        let mut move_steps = vec![];
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let down = (i + 1) % row_len;
                if cell.state == State::South && self.cells[down][j].state == State::Empty {
                    move_steps.push((i, j, State::Empty));
                    move_steps.push((down, j, State::South));
                }
            }
        }

        for (i, j, state) in &move_steps {
            self.cells[*i][*j].state = *state
        }
        !move_steps.is_empty()
    }

    fn move_step(&mut self) -> bool {
        self.move_steps_cnt += 1;
        let e = self.move_east();
        let s = self.move_south();
        e || s
    }

    fn dump(&self) {
        for row in self.cells.iter() {
            for cell in row {
                print!("{cell}");
            }
            println!();
        }
        println!("-----------------------");
    }
}
fn main() {
    let s = include_str!("input");
    let mut land = Land::new();
    for line in s.lines() {
        let mut cells = vec![];
        line.trim().chars().for_each(|c| {
            use State::*;
            let cell = match c {
                '.' => Cell { state: Empty },
                '>' => Cell { state: East },
                'v' => Cell { state: South },
                _ => unreachable!(),
            };
            cells.push(cell);
        });
        land.cells.push(cells);
    }
    // land.dump();
    while land.move_step() {}
    // land.dump();
    println!("part1: {}", land.move_steps_cnt);
}
