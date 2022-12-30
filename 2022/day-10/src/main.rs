static INPUT: &str = include_str!("input");

#[derive(PartialEq, Eq)]
enum Cmd {
    Addx,
    Noop,
}

impl Cmd {
    fn cycle(&self) -> i32 {
        match *self {
            Self::Addx => 2,
            Self::Noop => 1,
        }
    }
}

impl From<&str> for Cmd {
    fn from(cmd: &str) -> Self {
        match cmd {
            "addx" => Self::Addx,
            "noop" => Self::Noop,
            _ => unreachable!(),
        }
    }
}
struct Cpu {
    x: i32,
    cycle: i32,
    strengths: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            strengths: 0,
        }
    }

    fn signal_cycle(cycle: i32) -> bool {
        cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220
    }

    fn exec_cmd(&mut self, cmd: Cmd, value: i32, crt: &mut Crt) {
        let cycle = cmd.cycle();
        for _ in 0..cycle {
            self.cycle += 1;
            crt.draw(self);
            if Self::signal_cycle(self.cycle) {
                self.strengths += self.cycle * self.x;
            }
        }
        self.x += value;
    }

    fn exec_program(&mut self, program: &str, crt: &mut Crt) {
        program.lines().for_each(|line| {
            let mut cmd_line = line.split(' ');
            let cmd: Cmd = cmd_line.next().unwrap().into();
            let mut value = 0;
            if cmd == Cmd::Addx {
                value = cmd_line.next().unwrap().parse::<i32>().unwrap();
            }
            self.exec_cmd(cmd, value, crt);
        });
    }
}

#[derive(Debug)]
struct Crt {
    screen: [[u8; Crt::CRT_WIDTH]; Crt::CRT_HIGHT],
    x: usize,
    y: usize,
}

impl Crt {
    const CRT_WIDTH: usize = 40;
    const CRT_HIGHT: usize = 6;
    fn new() -> Self {
        Self {
            screen: [[0; Crt::CRT_WIDTH]; Crt::CRT_HIGHT],
            x: 0,
            y: 0,
        }
    }
    fn draw(&mut self, cpu: &Cpu) {
        if self.x >= Crt::CRT_HIGHT {
            return;
        }

        if self.y as i32 + 1 >= cpu.x && self.y as i32 + 1 < cpu.x + 3 {
            self.screen[self.x][self.y] = 1;
        }
        self.y += 1;
        if self.y >= Crt::CRT_WIDTH {
            self.y = 0;
            self.x += 1;
        }
    }

    fn dump(&self) {
        for row in &self.screen {
            for pixel in row {
                if *pixel > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    cpu.exec_program(INPUT, &mut crt);
    println!("part1: {}", cpu.strengths);
    println!("part2:");
    crt.dump();
}
#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let mut cpu = Cpu::new();
        let mut crt = Crt::new();
        cpu.exec_program(SAMPLE, &mut crt);
        println!("strengths: {}", cpu.strengths);
        assert_eq!(13140, cpu.strengths);
        crt.dump();
    }
}
