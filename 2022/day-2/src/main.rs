static INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
enum RoundStatus {
    Win,
    Loss,
    Draw,
}

impl RoundStatus {
    fn score(self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}
#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn win_lose_draw(self, opp: RPS) -> RoundStatus {
        match (self, opp) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                RoundStatus::Win
            }
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                RoundStatus::Loss
            }
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                RoundStatus::Draw
            }
        }
    }

    fn trans_to_status(self) -> RoundStatus {
        match self {
            RPS::Rock => RoundStatus::Loss,
            RPS::Paper => RoundStatus::Draw,
            RPS::Scissors => RoundStatus::Win,
        }
    }

    fn next(self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

impl From<&str> for RPS {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unreachable!(),
        }
    }
}

struct Round {
    opponent: RPS,
    me: RPS,
}

impl Round {
    fn new(opp: &str, me: &str) -> Self {
        Self {
            opponent: opp.into(),
            me: me.into(),
        }
    }

    fn score_part1(&self) -> i32 {
        self.me.score() + self.me.win_lose_draw(self.opponent).score()
    }

    fn score_part2(&self) -> i32 {
        let status = self.me.trans_to_status();
        let rps = match status {
            RoundStatus::Win => self.opponent.next(),
            RoundStatus::Loss => self.opponent.next().next(),
            RoundStatus::Draw => self.opponent,
        };
        rps.score() + status.score()
    }
}

struct Guide(Vec<Round>);

impl Guide {
    fn new(guide: Vec<Round>) -> Self {
        Self(guide)
    }

    fn score_part1(&self) -> i32 {
        self.0.iter().map(|round| round.score_part1()).sum()
    }

    fn score_part2(&self) -> i32 {
        self.0.iter().map(|round| round.score_part2()).sum()
    }
}

fn parse_strategy_guide(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            Round::new(line.next().unwrap(), line.next().unwrap())
        })
        .collect()
}

fn main() {
    let guide = Guide::new(parse_strategy_guide(INPUT));
    println!("part1: {}", guide.score_part1());
    println!("part2: {}", guide.score_part2());
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let guide = Guide::new(parse_strategy_guide(SAMPLE));
        assert_eq!(guide.score_part1(), 15);
    }
    #[test]
    fn test_part2_sample() {
        let guide = Guide::new(parse_strategy_guide(SAMPLE));
        assert_eq!(guide.score_part2(), 12);
    }
}
