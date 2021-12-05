fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut cmd = Vec::<(&str, i32)>::new();
    for line in s.lines() {
        let mut iter = line.split(' ');
        let op = iter.next().unwrap();
        let data = iter.next().unwrap();
        cmd.push((op, data.parse::<i32>().unwrap()));
    }
    let mut aim = 0;
    let mut hoz = 0;
    let mut depth = 0;
    let mut depth2 = 0;
    for (k, v) in cmd.clone() {
        match k {
            "forward" => {
                hoz += v;
                depth2 += aim * v;
            }
            "up" => {
                depth -= v;
                aim -= v;
            }
            "down" => {
                depth += v;
                aim += v;
            }
            _ => unreachable!(),
        };
    }
    println!("hoz:{}, dep:{}, multi:{}", hoz, depth, hoz * depth);
    println!("hoz:{}, dep2:{}, multi:{}", hoz, depth2, hoz * depth2);
}
