use std::collections::HashMap;

fn find_sum(v: &Vec<i32>, sum: i32) -> (i32, i32) {
    let mut m = HashMap::new();
    for n in v {
        let k = sum - *n;
        match m.get(&k) {
            Some(_) => {
                return (k, *n);
            }
            None => {
                m.insert(n, true);
            }
        }
    }
    (0, 0)
}
fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut v = vec![];
    for line in s.lines() {
        let n = line.parse::<i32>().unwrap();
        v.push(n);
    }
    let ans = find_sum(&v, 2020);
    println!("tow entries: {} * {} = {}", ans.0, ans.1, ans.0 * ans.1);
    for n in &v {
        let sum = 2020 - *n;
        let ans = find_sum(&v, sum);
        if ans.0 > 0 && ans.1 > 0 {
            println!(
                "three entries: {} * {} * {} = {}",
                *n,
                ans.0,
                ans.1,
                *n * ans.0 * ans.1
            );
            break;
        }
    }
}
