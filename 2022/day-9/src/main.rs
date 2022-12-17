use std::collections::HashMap;
static INPUT: &str = include_str!("input");

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Postion(i32, i32);

impl Postion {
    fn left(&mut self) {
        self.0 -= 1;
    }
    fn right(&mut self) {
        self.0 += 1;
    }
    fn up(&mut self) {
        self.1 += 1;
    }
    fn down(&mut self) {
        self.1 -= 1;
    }

    fn distance(p1: Postion, p2: Postion) -> (i32, i32) {
        ((p1.0 - p2.0), (p1.1 - p2.1))
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: Postion,
    tail: Vec<Postion>,
    tail_visited: HashMap<Postion, u32>,
}

impl Rope {
    fn new(tail_num: usize) -> Self {
        let mut rope = Rope {
            tail: vec![Postion::default(); tail_num],
            ..Default::default()
        };
        rope.tail_visited.insert(Postion::default(), 1);
        rope
    }

    fn move_tail(&mut self) {
        let tail_len = self.tail.len();
        let mut head = self.head;
        for (i, tail) in self.tail.iter_mut().enumerate() {
            let (x, y) = Postion::distance(head, *tail);

            let mut no_move = true;
            if x.abs() > 1 {
                tail.0 += if x > 0 { 1 } else { -1 };
                if y.abs() == 1 {
                    tail.1 = head.1;
                }
                no_move = false;
            }
            if y.abs() > 1 {
                tail.1 += if y > 0 { 1 } else { -1 };
                if x.abs() == 1 {
                    tail.0 = head.0;
                }
                no_move = false;
            }
            if no_move {
                return;
            }
            head = *tail;
            if i == tail_len - 1 {
                self.tail_visited.insert(*tail, 1);
            }
        }
    }

    fn left(&mut self) {
        self.head.left();
        self.move_tail();
    }
    fn right(&mut self) {
        self.head.right();
        self.move_tail();
    }
    fn up(&mut self) {
        self.head.up();
        self.move_tail();
    }
    fn down(&mut self) {
        self.head.down();
        self.move_tail();
    }

    fn simulating(&mut self, input: &str) {
        input.lines().for_each(|line| {
            let mut iter = line.split(' ');
            let dir = iter.next().unwrap();
            let steps = iter.next().unwrap().parse::<u32>().unwrap();
            for _ in 0..steps {
                match dir {
                    "R" => self.right(),
                    "L" => self.left(),
                    "U" => self.up(),
                    "D" => self.down(),
                    _ => unreachable!(),
                }
            }
        });
    }
}

fn main() {
    let mut rope = Rope::new(1);
    rope.simulating(INPUT);
    println!("part1: {}", rope.tail_visited.len());
    let mut rope = Rope::new(9);
    rope.simulating(INPUT);
    println!("part2: {}", rope.tail_visited.len());
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    static SAMPLE2: &str = include_str!("sample2");
    #[test]
    fn test_part1_sample() {
        let mut rope = Rope::new(1);
        rope.simulating(SAMPLE);
        assert_eq!(13, rope.tail_visited.len());
    }
    #[test]
    fn test_part2_sample() {
        let mut rope = Rope::new(9);
        rope.simulating(SAMPLE);
        assert_eq!(1, rope.tail_visited.len());
        let mut rope = Rope::new(9);
        rope.simulating(SAMPLE2);
        assert_eq!(36, rope.tail_visited.len());
    }
}
