fn find_increase(sample: &Vec<i32>, window: usize) -> i32 {
    let mut v = vec![];
    for (i, n) in sample.iter().enumerate() {
        if i >= window - 1 {
            let mut sum = 0;
            for j in 0..window {
                sum += sample.get(i - j).unwrap();
            }
            v.push(sum);
        }
    }
    let mut res = 0;
    for i in 1..v.len() {
        if v.get(i) > v.get(i - 1) {
            res += 1;
        }
    }
    res
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut v = vec![];
    for line in s.lines() {
        let n = line.parse::<i32>().unwrap();
        v.push(n);
    }
    let res = find_increase(&v, 1);
    println!("{}", res);
    let res = find_increase(&v, 3);
    println!("{}", res);
}
