use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

struct Dice {
    sided: usize,
    rolled: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        let mut step = 0;
        for _ in 0..3 {
            step += self.rolled % self.sided + 1;
            self.rolled += 1;
        }
        step
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    pos: usize,
    score: usize,
    win_score: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Self {
            pos,
            score: 0,
            win_score: 0,
        }
    }
    fn win(&self) -> bool {
        self.score >= self.win_score
    }

    fn add_score(&mut self, score: usize) {
        self.score += score;
        self.pos = score;
    }
}

const TRACK_LEN: usize = 10;
fn track_score(start: usize, step: usize) -> usize {
    // map 1..10 to 0..9 for mod
    (start - 1 + step) % TRACK_LEN + 1
}

fn main() {
    let s = include_str!("input");
    let mut player1 = Player::new(0);
    let mut player2 = Player::new(0);
    for (n, line) in s.lines().enumerate() {
        let pos = line
            .split(':')
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        if n == 0 {
            player1 = Player::new(pos);
        } else if n == 1 {
            player2 = Player::new(pos);
        }
    }
    part1(player1, player2);
    part2(player1, player2);
}

lazy_static! {
    static ref DP: Mutex::<HashMap<String, (usize, usize)>> = Mutex::new(HashMap::new());
}

fn get_dp_key(
    round: usize,
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
) -> String {
    format!("{}-{}-{}-{}-{}", round, p1_pos, p2_pos, p1_score, p2_score)
}

fn win_universe(round: usize, p1: &Player, p2: &Player) -> (usize, usize) {
    let dp_key = get_dp_key(round, p1.pos, p2.pos, p1.score, p2.score);
    if let Some(v) = DP.lock().unwrap().get(&dp_key) {
        return *v;
    }
    if p1.win() {
        return (1, 0);
    }
    if p2.win() {
        return (0, 1);
    }

    let mut p1_win = 0;
    let mut p2_win = 0;
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let mut p = if round == 0 { *p1 } else { *p2 };
                let score = track_score(p.pos, i + j + k);
                p.add_score(score);
                let (w1, w2) = if round == 0 {
                    win_universe(1 - round, &p, p2)
                } else {
                    win_universe(1 - round, p1, &p)
                };
                p1_win += w1;
                p2_win += w2;
            }
        }
    }
    *DP.lock().unwrap().entry(dp_key).or_default() = (p1_win, p2_win);
    (p1_win, p2_win)
}

fn part2(mut p1: Player, mut p2: Player) {
    p1.win_score = 21;
    p2.win_score = 21;

    let (w1, w2) = win_universe(0, &p1, &p2);
    // println!("p1_win_universe: {:?}, p2_win_universe: {:?}", w1, w2,);
    println!("part2: {}", std::cmp::max(w1, w2));
}

fn part1(mut p1: Player, mut p2: Player) {
    p1.win_score = 1000;
    p2.win_score = 1000;
    let mut dice = Dice {
        sided: 100,
        rolled: 0,
    };

    loop {
        if p1.win() {
            println!("part1: {}", p2.score * dice.rolled);
            break;
        }
        if p2.win() {
            println!("part1: {}", p1.score * dice.rolled);
            break;
        }

        p1.add_score(track_score(p1.pos, dice.roll()));
        p2.add_score(track_score(p2.pos, dice.roll()));
    }
}
