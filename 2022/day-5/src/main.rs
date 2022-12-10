static INPUT: &str = include_str!("input");

enum Crane {
    C9000,
    C9001,
}

#[derive(Debug, Clone, Copy)]
struct Rearrangement {
    cnt: i32,
    from: i32,
    to: i32,
}

impl Rearrangement {
    fn new(cnt: i32, from: i32, to: i32) -> Self {
        Self { cnt, from, to }
    }

    fn parse_arrange(s: &str) -> Result<Self, &str> {
        let v = s
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        if v.len() != 3 {
            return Err("parse arrangement error");
        };
        Ok(Self::new(
            *v.first().unwrap(),
            *v.get(1).unwrap() - 1,
            *v.get(2).unwrap() - 1,
        ))
    }
}
#[derive(Debug, Clone)]
struct Crates {
    stacks: Vec<Vec<char>>,
    rearrangement: Vec<Rearrangement>,
}

impl Crates {
    fn new() -> Self {
        Self {
            stacks: vec![],
            rearrangement: vec![],
        }
    }

    // stack_index starts from 0
    fn add_crate(&mut self, stack_index: usize, c: char) {
        if (stack_index + 1) > self.stacks.len() {
            for _ in 0..(stack_index + 1) - self.stacks.len() {
                self.stacks.push(vec![]);
            }
        }
        match self.stacks.get_mut(stack_index) {
            Some(stack) => stack.insert(0, c),
            None => unreachable!(),
        }
    }

    fn parse_input(input: &str) -> Self {
        let mut crates = Self::new();
        let mut stack_end = false;
        input.lines().for_each(|line| {
            if line.is_empty() {
                stack_end = true;
            } else if stack_end {
                // parse arrangement
                crates
                    .rearrangement
                    .push(Rearrangement::parse_arrange(line).unwrap());
            } else {
                // parse stacks
                for (i, c) in line.chars().enumerate() {
                    if !stack_end && i % 4 == 1 && c.is_ascii_alphabetic() {
                        crates.add_crate(i / 4, c)
                    }
                }
            }
        });
        crates
    }

    fn rearrangement_procedure_crane_9000(&mut self, arrange: Rearrangement) {
        let cnt = arrange.cnt;
        let from = arrange.from as usize;
        let to = arrange.to as usize;
        for _ in 0..cnt {
            if let Some(stack) = self.stacks.get_mut(from) {
                if let Some(elem) = stack.pop() {
                    if let Some(stack) = self.stacks.get_mut(to) {
                        stack.push(elem);
                    }
                }
            }
        }
    }
    fn rearrangement_procedure_crane_9001(&mut self, arrange: Rearrangement) {
        let cnt = arrange.cnt;
        let from = arrange.from as usize;
        let to = arrange.to as usize;
        if let Some(stack) = self.stacks.get_mut(from) {
            let mut v = vec![];
            for _ in 0..cnt {
                if let Some(elem) = stack.pop() {
                    v.insert(0, elem);
                }
            }
            if let Some(stack) = self.stacks.get_mut(to) {
                stack.extend(v);
            }
        }
    }

    fn rearrangement_procedure(&mut self, crane: Crane) {
        self.rearrangement
            .clone()
            .iter()
            .for_each(|arrange| match crane {
                Crane::C9000 => self.rearrangement_procedure_crane_9000(*arrange),
                Crane::C9001 => self.rearrangement_procedure_crane_9001(*arrange),
            });
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>()
    }
}

fn main() {
    let mut crates = Crates::parse_input(INPUT);
    crates.rearrangement_procedure(Crane::C9000);
    println!("part1: {}", crates.top_crates());
    let mut crates = Crates::parse_input(INPUT);
    crates.rearrangement_procedure(Crane::C9001);
    println!("part2: {}", crates.top_crates());
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let mut crates = Crates::parse_input(SAMPLE);
        crates.rearrangement_procedure(Crane::C9000);
        assert_eq!("CMZ".to_string(), crates.top_crates());
    }
    #[test]
    fn test_part2_sample() {
        let mut crates = Crates::parse_input(SAMPLE);
        crates.rearrangement_procedure(Crane::C9001);
        assert_eq!("MCD".to_string(), crates.top_crates());
    }
}
