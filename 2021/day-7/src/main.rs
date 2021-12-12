use std::collections::HashMap;

fn main() {
    let line = include_str!("input");

    let mut max_position = 0;
    let mut position_map = HashMap::new();
    for n in line.split(',') {
        let p = n.parse::<i32>().unwrap();
        max_position = std::cmp::max(p, max_position);
        if let Some(n) = position_map.get_mut(&p) {
            *n += 1;
        } else {
            position_map.insert(p, 1);
        }
    }
    let mut ans_position_part1 = 0;
    let mut ans_position_part2 = 0;
    let mut ans_fuel_part1 = 0;
    let mut ans_fuel_part2 = 0;
    for i in 0..=max_position {
        let mut fuel_part1 = 0;
        let mut fuel_part2 = 0;
        for (k, v) in &position_map {
            if i != *k {
                let distance = (*k - i).abs();
                fuel_part1 += *v * distance;
                fuel_part2 += *v * distance * (distance + 1) / 2;
            }
        }
        if fuel_part1 < ans_fuel_part1 || ans_fuel_part1 == 0 {
            ans_fuel_part1 = fuel_part1;
            ans_position_part1 = i;
        }

        if fuel_part2 < ans_fuel_part2 || ans_fuel_part2 == 0 {
            ans_fuel_part2 = fuel_part2;
            ans_position_part2 = i;
        }
    }
    println!(
        "part 1: position: {}, fuel: {}",
        ans_position_part1, ans_fuel_part1
    );
    println!(
        "part 2: position: {}, fuel: {}",
        ans_position_part2, ans_fuel_part2
    );
}
