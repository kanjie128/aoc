static INPUT: &str = include_str!("input");

fn main() {
    let max = part1(INPUT);
    println!("part1: {max:?}");

    let total = part2(INPUT);
    println!("part2: {total:?}");
}

fn calories_elf_carray(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|calory| calory.parse::<i32>().unwrap())
                .sum()
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    *calories_elf_carray(input).iter().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut calories = calories_elf_carray(input);
    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    static SAMPLE: &str = include_str!("sample");
    use super::*;
    #[test]
    fn test_part1_sample() {
        let max = part1(SAMPLE);
        assert_eq!(max, 24000);
    }
    #[test]
    fn test_part2_sample() {
        let max = part2(SAMPLE);
        assert_eq!(max, 45000);
    }
}
