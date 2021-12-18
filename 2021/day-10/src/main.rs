#[derive(Debug)]
enum LinePattern {
    Incomplete,
    Corrupted,
}

fn main() {
    let s = include_str!("input");

    let mut incomplete_lines = vec![];
    let mut part1_score = 0;
    for line in s.lines() {
        let score = corrupted_score(line);
        if score == 0 {
            incomplete_lines.push(line);
        }
        part1_score += score;
    }
    println!("part 1: {}", part1_score);

    let mut part2_score = vec![];
    for line in incomplete_lines {
        let score = incomplete_score(line);
        // println!("incomplete:{}, score:{}", line, score);
        part2_score.push(score);
    }
    part2_score.sort_unstable();
    println!(
        "part 2: {}",
        part2_score.get(part2_score.len() / 2).unwrap()
    );
}

fn incomplete_score(line: &str) -> i64 {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
            continue;
        }
        let open_char = stack.pop().unwrap();
        if get_close(open_char) != c {
            panic!("input line ({}) is corrupted", line);
        }
    }
    stack.reverse();
    let mut score = 0;
    for c in stack {
        score = score * 5 + ch_score(get_close(c), LinePattern::Incomplete);
    }
    score
}

fn corrupted_score(s: &str) -> i64 {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if is_open(c) {
            stack.push(c);
            continue;
        }
        if let Some(v) = stack.last() {
            if get_close(*v) == c {
                stack.pop();
            } else {
                return ch_score(c, LinePattern::Corrupted);
            }
        } else {
            return ch_score(c, LinePattern::Corrupted);
        }
    }
    0
}

fn get_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("{} not valid", c),
    }
}

fn ch_score(c: char, p: LinePattern) -> i64 {
    match p {
        LinePattern::Incomplete => match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!("{} not valid", c),
        },
        LinePattern::Corrupted => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!("{} not valid", c),
        },
    }
}

fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}
