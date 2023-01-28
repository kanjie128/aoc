use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse_input(input: &str) -> Vec<Self> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_str(line).expect("input should be json format"))
            .collect()
    }

    fn list_map<F, T>(&self, map: F) -> T
    where
        F: Fn(&[Packet]) -> T,
    {
        match self {
            Self::Number(n) => map(&[Self::Number(*n)]),
            Self::List(l) => map(&l[..]),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::*;
        match (self, other) {
            (Number(n1), Number(n2)) => n1.partial_cmp(n2),
            (left, right) => left.list_map(|l| right.list_map(|r| l.partial_cmp(r))),
        }
    }
}

#[derive(Copy, Clone)]
struct Packets<'a> {
    packets: &'a Vec<Packet>,
    step: usize,
    start: usize,
}

impl<'a> Packets<'a> {
    fn new(packets: &'a Vec<Packet>) -> Self {
        Self {
            packets,
            // iterator every 2 packet
            step: 2,
            start: 0,
        }
    }
    fn len(&self) -> usize {
        self.packets.len()
    }

    fn part1(&self) -> usize {
        self.enumerate()
            .filter_map(|(index, (p1, p2))| if p1 < p2 { Some(index + 1) } else { None })
            .sum()
    }

    fn part2(&self) -> usize {
        use Packet::*;
        let mut packets = self.packets.clone();
        let div = vec![
            List(vec![List(vec![Number(2)])]),
            List(vec![List(vec![Number(6)])]),
        ];
        packets.append(&mut div.clone());
        packets.sort();
        let mut key = 1;
        div.iter().for_each(|p| {
            key *= packets.binary_search(p).unwrap() + 1;
        });
        key
    }
}

impl<'a> Iterator for Packets<'a> {
    type Item = (&'a Packet, &'a Packet);
    fn next(&mut self) -> Option<Self::Item> {
        if self.len() < self.step || self.start + self.step > self.len() {
            return None;
        }
        let pair = Some((
            self.packets.get(self.start).unwrap(),
            self.packets.get(self.start + 1).unwrap(),
        ));

        self.start += self.step;

        pair
    }
}

static INPUT: &str = include_str!("input");

fn main() {
    let packets = Packet::parse_input(INPUT);
    let packets = Packets::new(&packets);
    assert_eq!(packets.len() % 2, 0);
    println!("part1: {}", packets.part1());
    println!("part2: {}", packets.part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let packets = Packet::parse_input(SAMPLE);
        let packets = Packets::new(&packets);
        assert_eq!(packets.len() % 2, 0);

        let sum = packets.part1();
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_part2_sample() {
        let packets = Packet::parse_input(SAMPLE);
        let packets = Packets::new(&packets);
        assert_eq!(packets.len() % 2, 0);

        let key = packets.part2();
        assert_eq!(key, 140)
    }
}
