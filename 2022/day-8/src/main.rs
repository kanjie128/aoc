struct TreeMap(Vec<Vec<u8>>);

static INPUT: &str = include_str!("input");

impl TreeMap {
    fn parse_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
                .collect::<Vec<_>>(),
        )
    }

    fn is_visible_at(&self, i: usize, j: usize, tree: u8) -> bool {
        if let Some(row) = self.0.get(i) {
            if let Some(col) = row.get(j) {
                return *col < tree;
            }
        }
        true
    }

    fn visible_trees(&self) -> (i32, i32) {
        let mut visible = 0;
        let mut highest_scenic_score = 0;
        for (i, row) in self.0.iter().enumerate() {
            for (j, tree) in row.iter().enumerate() {
                if i == 0 || i == self.0.len() - 1 || j == 0 || j == row.len() - 1 {
                    visible += 1;
                } else {
                    let mut is_visible = false;
                    // up
                    let mut up = i as i32 - 1;
                    let mut up_score = 1;
                    while up >= 0 && self.is_visible_at(up as usize, j, *tree) {
                        up -= 1;
                        up_score += 1;
                    }
                    if up < 0 {
                        is_visible = true;
                        up_score -= 1;
                    }
                    // down
                    let mut down = i + 1;
                    let mut down_score = 1;
                    while down < self.0.len() && self.is_visible_at(down, j, *tree) {
                        down += 1;
                        down_score += 1;
                    }
                    if down >= self.0.len() {
                        is_visible = true;
                        down_score -= 1;
                    }
                    // left
                    let mut left = j as i32 - 1;
                    let mut left_score = 1;
                    while left >= 0 && self.is_visible_at(i, left as usize, *tree) {
                        left -= 1;
                        left_score += 1;
                    }
                    if left < 0 {
                        is_visible = true;
                        left_score -= 1;
                    }
                    // right
                    let mut right = j + 1;
                    let mut right_score = 1;
                    while right < row.len() && self.is_visible_at(i, right, *tree) {
                        right += 1;
                        right_score += 1;
                    }
                    if right >= row.len() {
                        is_visible = true;
                        right_score -= 1;
                    }
                    if is_visible {
                        visible += 1;
                    }
                    let score = up_score * down_score * left_score * right_score;
                    if score > highest_scenic_score {
                        highest_scenic_score = score;
                    };
                }
            }
        }
        (visible, highest_scenic_score)
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for ele in self.0.iter() {
            for tree in ele {
                print!("{tree}");
            }
            println!();
        }
        println!("----------");
    }
}

fn main() {
    let tree_map = TreeMap::parse_input(INPUT);
    let (visible_cnt, hightest) = tree_map.visible_trees();
    println!("part1: {}", visible_cnt);
    println!("part2: {}", hightest);
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let tree_map = TreeMap::parse_input(SAMPLE);
        tree_map.dump();
        let (visible_cnt, hightest) = tree_map.visible_trees();
        assert_eq!(21, visible_cnt);
        assert_eq!(8, hightest);
    }
}
