use std::collections::HashMap;

static INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
enum MarkerType {
    StartPacket,
    StartMessage,
}

impl MarkerType {
    fn marker_len(self) -> usize {
        match self {
            Self::StartPacket => 4,
            Self::StartMessage => 14,
        }
    }
}

struct Buffer<'a>(&'a str);

impl<'a> Buffer<'a> {
    fn new(s: &'a str) -> Self {
        Self(s)
    }

    fn marker_postion(&self, marker_type: MarkerType) -> usize {
        let mut character_pos_map = HashMap::<char, usize>::new();
        let mut curr_marker_len = 0;
        for (i, c) in self.0.chars().enumerate() {
            if let Some(pos) = character_pos_map.get_mut(&c) {
                let old_pos = *pos;
                *pos = i;
                curr_marker_len = i - old_pos;
                character_pos_map.retain(|_, p| *p > old_pos);
            } else {
                curr_marker_len += 1;
                if curr_marker_len >= marker_type.marker_len() {
                    return i + 1;
                }
                character_pos_map.insert(c, i);
            }
        }
        0
    }
}

fn main() {
    let buffer = Buffer::new(INPUT);
    println!("part1: {}", buffer.marker_postion(MarkerType::StartPacket));
    println!("part2: {}", buffer.marker_postion(MarkerType::StartMessage));
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let sample_expect_answer = [7, 5, 6, 10, 11];
        for (i, line) in SAMPLE.lines().enumerate() {
            let buffer = Buffer::new(line);
            println!("sample:{line}");
            assert_eq!(
                sample_expect_answer[i],
                buffer.marker_postion(MarkerType::StartPacket)
            );
        }
    }
    #[test]
    fn test_part2_sample() {
        let sample_expect_answer = [19, 23, 23, 29, 26];
        for (i, line) in SAMPLE.lines().enumerate() {
            let buffer = Buffer::new(line);
            println!("sample:{line}");
            assert_eq!(
                sample_expect_answer[i],
                buffer.marker_postion(MarkerType::StartMessage)
            );
        }
    }
}
