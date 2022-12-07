static INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
struct Sections(i32, i32);

impl Sections {
    fn section_fully_contain_or_in(&s1: &Sections, s2: &Sections) -> bool {
        (s1.0 <= s2.0 && s1.1 >= s2.1) || (s1.0 >= s2.0 && s1.1 <= s2.1)
    }

    fn section_overlap(s1: &Sections, s2: &Sections) -> bool {
        Self::section_fully_contain_or_in(s1, s2)
            || (s1.0 <= s2.0 && s1.1 >= s2.0)
            || (s1.0 <= s2.1 && s1.1 >= s2.1)
    }
}

struct Assignment(Vec<(Sections, Sections)>);

impl Assignment {
    fn parse_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let pair = line
                        .split(',')
                        .map(|section| {
                            let section = section
                                .split('-')
                                .map(|n| n.parse::<i32>().unwrap())
                                .collect::<Vec<_>>();
                            Sections(*section.first().unwrap(), *section.get(1).unwrap())
                        })
                        .collect::<Vec<_>>();
                    (*pair.first().unwrap(), *pair.get(1).unwrap())
                })
                .collect::<Vec<_>>(),
        )
    }

    fn count_duplicate<F>(&self, conditon: F) -> usize
    where
        F: Fn(&Sections, &Sections) -> bool,
    {
        self.0
            .iter()
            .filter(|&&pair| conditon(&pair.0, &pair.1))
            .count()
    }
}

fn main() {
    let assignment = Assignment::parse_input(INPUT);
    println!(
        "part1: {}",
        assignment.count_duplicate(Sections::section_fully_contain_or_in)
    );
    println!(
        "part2: {}",
        assignment.count_duplicate(Sections::section_overlap)
    );
}

#[cfg(test)]
mod test {
    static SAMPLE: &str = include_str!("sample");
    use super::*;
    #[test]
    fn test_part1_sample() {
        let assignment = Assignment::parse_input(SAMPLE);
        assert_eq!(
            assignment.count_duplicate(Sections::section_fully_contain_or_in),
            2
        );
    }

    #[test]
    fn test_part2_sample() {
        let assignment = Assignment::parse_input(SAMPLE);
        assert_eq!(assignment.count_duplicate(Sections::section_overlap), 4);
    }
}
