use std::collections::HashSet;
static INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Rucksack<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> Rucksack<'a> {
    fn find_common_item_type(&self) -> HashSet<char> {
        self.first
            .chars()
            .filter(|c| self.second.find(*c).is_some())
            .collect()
    }

    fn parse_input(input: &str) -> Vec<Rucksack> {
        input
            .lines()
            .map(|line| {
                let line = line.split_at(line.len() / 2);
                Rucksack {
                    first: line.0,
                    second: line.1,
                }
            })
            .collect()
    }

    fn sum_priority(rucksacks: &[Self]) -> i32 {
        rucksacks
            .iter()
            .map(|r| {
                r.find_common_item_type()
                    .iter()
                    .map(|c| priority(*c))
                    .sum::<i32>()
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct ElfGroup<'a>(Vec<&'a str>);

impl<'a> ElfGroup<'a> {
    fn new() -> Self {
        Self(vec![])
    }
    fn parse_input(input: &'a str) -> Vec<Self> {
        let mut elf_groups = vec![];
        let mut group = ElfGroup::new();
        for (i, line) in input.lines().enumerate() {
            group.0.push(line);
            if (i + 1) % 3 == 0 {
                elf_groups.push(group.clone());
                group.0.clear();
            }
        }
        elf_groups
    }

    fn find_common_item_type(&self) -> HashSet<char> {
        let line0 = *self.0.first().unwrap();
        let line1 = *self.0.get(1).unwrap();
        let line2 = *self.0.get(2).unwrap();
        line0
            .chars()
            .filter(|c| line1.find(*c).is_some() && line2.find(*c).is_some())
            .collect()
    }

    fn sum_priority(groups: &[Self]) -> i32 {
        groups
            .iter()
            .map(|group| {
                group
                    .find_common_item_type()
                    .iter()
                    .map(|c| priority(*c))
                    .sum::<i32>()
            })
            .sum()
    }
}

fn priority(c: char) -> i32 {
    match c {
        c @ 'a'..='z' => c as i32 - 'a' as i32 + 1,
        c @ 'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => unreachable!(),
    }
}

fn main() {
    let rucksacks = Rucksack::parse_input(INPUT);
    let sum = Rucksack::sum_priority(&rucksacks);
    println!("part1: {sum}");

    let groups = ElfGroup::parse_input(INPUT);
    let sum = ElfGroup::sum_priority(&groups);
    println!("part2: {sum}");
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let rucksacks = Rucksack::parse_input(SAMPLE);
        let sum = Rucksack::sum_priority(&rucksacks);
        assert_eq!(sum, 157);
    }
    #[test]
    fn test_part2_sample() {
        let groups = ElfGroup::parse_input(SAMPLE);
        let sum = ElfGroup::sum_priority(&groups);
        assert_eq!(sum, 70);
    }
}
