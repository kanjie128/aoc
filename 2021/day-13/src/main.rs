use std::{error::Error, vec};

#[derive(Debug, Clone)]
struct Paper {
    dots: Vec<Vec<i32>>,
}

impl Paper {
    fn new(x: i32, y: i32) -> Self {
        Self {
            dots: vec![vec![0; x as usize]; y as usize],
        }
    }

    fn insert(&mut self, x: i32, y: i32) {
        // self.extend(x, y);
        let p = self
            .dots
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();
        *p = 1;
    }

    fn dump(&self) {
        self.dots.iter().for_each(|row| {
            row.iter().for_each(|p| {
                print!("{} ", if *p == 1 { "#" } else { "." });
            });
            println!();
        });
    }

    fn get_point(&self, x: usize, y: usize) -> i32 {
        self.dots[y][x]
    }

    fn fold(&mut self, instruct: &FoldInstruct) {
        match instruct.axis {
            Axis::Y => {
                let y = instruct.value;
                let x = self.dots[0].len();
                (0..y).for_each(|row| {
                    (0..x).for_each(|col| {
                        if self.get_point(col, row) == 0 {
                            let symmetric = 2 * y - row;
                            if self.get_point(col, symmetric) == 1 {
                                *self.dots.get_mut(row).unwrap().get_mut(col).unwrap() = 1;
                            }
                        }
                    });
                });
                self.dots.drain(y..);
            }
            Axis::X => {
                let x = instruct.value;
                let y = self.dots.len();
                (0..y).for_each(|row| {
                    (0..x).for_each(|col| {
                        if self.get_point(col, row) == 0 {
                            let symmetric = 2 * x - col;
                            if self.get_point(symmetric, row) == 1 {
                                *self.dots.get_mut(row).unwrap().get_mut(col).unwrap() = 1;
                            }
                        }
                    });
                });
                for row in self.dots.iter_mut() {
                    row.drain(x..);
                }
            }
        }
    }

    fn visible(&self) -> i32 {
        self.dots.iter().flatten().sum()
    }

    fn part1(&mut self, ins: &FoldInstruct) -> i32 {
        self.fold(ins);
        self.visible()
    }

    fn part2(&mut self, inss: &[FoldInstruct]) -> i32 {
        for ins in inss {
            self.fold(ins);
        }
        self.dump();
        self.visible()
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

impl TryFrom<&str> for Axis {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "x" {
            Ok(Self::X)
        } else if value == "y" {
            Ok(Self::Y)
        } else {
            Err(format!("fold axis {:?} is not support", value))
        }
    }
}

#[derive(Debug)]
struct FoldInstruct {
    axis: Axis,
    value: usize,
}

impl FoldInstruct {
    fn new(axis: Axis, value: usize) -> Self {
        Self { axis, value }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let s = include_str!("input");

    let mut dots_end = false;

    let mut dots: Vec<(i32, i32)> = vec![];
    let mut fold_instructs: Vec<FoldInstruct> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    for line in s.trim().lines() {
        if line.is_empty() {
            dots_end = true;
            continue;
        }
        if !dots_end {
            let p = line.split(',').collect::<Vec<_>>();
            if p.len() == 2 {
                let x = p[0].parse::<i32>()?;
                let y = p[1].parse::<i32>()?;
                dots.push((x, y));
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            } else {
                panic!("input dots len error: {p:?}");
            }
        } else {
            let fold_rule = line.split(' ').collect::<Vec<_>>();
            if fold_rule.len() != 3 {
                panic!("input fold rule error: {fold_rule:?}");
            }
            let fold_rule = fold_rule[2].split('=').collect::<Vec<_>>();
            if fold_rule.len() != 2 {
                panic!("input fold rule error: {fold_rule:?}");
            }
            let fold_axis = fold_rule[0];
            let fold_point = fold_rule[1].parse::<usize>()?;
            fold_instructs.push(FoldInstruct::new(fold_axis.try_into()?, fold_point));
        }
    }

    let mut paper = Paper::new(max_x + 1, max_y + 1);
    for (x, y) in dots {
        paper.insert(x, y);
    }

    let mut paper2 = paper.clone();
    println!("part1: {}", paper.part1(&fold_instructs[0]));
    println!("--------------------------");
    println!("part2: {}", paper2.part2(&fold_instructs));

    Ok(())
}
