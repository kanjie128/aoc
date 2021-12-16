use itertools::Itertools;

fn main() {
    let s = include_str!("input");
    let mut sum = 0;
    let mut count = 0;
    for line in s.lines() {
        let digs = line.split('|').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(digs.len(), 2);
        //
        let mut zero = String::new();
        let mut _one = String::new();
        let mut two = String::new();
        let mut three = String::new();
        let mut four = String::new();
        let mut five = String::new();
        let mut six = String::new();
        let mut seven = String::new();
        let mut _eight = String::new();
        let mut nine = String::new();
        // find 1 4 7 8
        for v in digs.get(0).unwrap().split(' ') {
            match v.len() {
                // digit 1
                2 => {
                    _one = v.chars().sorted().collect::<String>();
                }
                // '4'
                4 => {
                    //bcdf
                    four = v.chars().sorted().collect::<String>();
                }
                // '7'
                3 => {
                    // acf
                    seven = v.chars().sorted().collect::<String>();
                }
                // '8'
                7 => {
                    //abcdefg
                    _eight = v.chars().sorted().collect::<String>();
                }
                _ => (),
            }
        }
        // find other
        for v in digs.get(0).unwrap().split(' ') {
            match v.len() {
                5 => {
                    // 2/3/5
                    //find 3
                    let mut find = true;
                    for i in seven.as_str().chars() {
                        let mut temp = false;
                        for j in v.chars() {
                            if i == j {
                                temp = true;
                            }
                        }
                        if !temp {
                            find = false;
                            break;
                        }
                    }
                    if find {
                        three = v.chars().sorted().collect::<String>();
                        continue;
                    }
                    // find 2 or 5
                    let mut same = 0;
                    for i in four.as_str().chars() {
                        for j in v.chars() {
                            if i == j {
                                same += 1;
                            }
                        }
                    }
                    if same == 2 {
                        two = v.chars().sorted().collect::<String>();
                    } else if same == 3 {
                        five = v.chars().sorted().collect::<String>();
                    } else {
                        unreachable!("not 2 or 5?");
                    }
                }
                6 => {
                    //find 9
                    let mut find = true;
                    for i in four.as_str().chars() {
                        let mut temp = false;
                        for j in v.chars() {
                            if i == j {
                                temp = true;
                            }
                        }
                        if !temp {
                            find = false;
                            break;
                        }
                    }
                    if find {
                        nine = v.chars().sorted().collect::<String>();
                        continue;
                    }
                    // find 0 or 6
                    let mut find = true;
                    for i in seven.as_str().chars() {
                        let mut temp = false;
                        for j in v.chars() {
                            if i == j {
                                temp = true;
                            }
                        }
                        if !temp {
                            find = false;
                            break;
                        }
                    }
                    if find {
                        zero = v.chars().sorted().collect::<String>();
                        continue;
                    } else {
                        six = v.chars().sorted().collect::<String>();
                    }
                }
                _ => (),
            }
        }

        //
        let mut s = "".to_string();
        for v in digs.get(1).unwrap().split(' ') {
            match v.len() {
                // digit 1
                2 => {
                    s.push('1');
                    count += 1;
                }
                // '4'
                4 => {
                    s.push('4');
                    count += 1;
                }
                // '7'
                3 => {
                    s.push('7');
                    count += 1
                }
                // '8'
                7 => {
                    s.push('8');
                    count += 1;
                }
                // 0/6/9
                6 => {
                    let t = v.chars().sorted().collect::<String>();
                    if t.as_str() == zero.as_str() {
                        s.push('0');
                    } else if t.as_str() == six.as_str() {
                        s.push('6');
                    } else if t.as_str() == nine.as_str() {
                        s.push('9');
                    } else {
                        unreachable!("what happen?")
                    }
                }
                5 => {
                    let t = v.chars().sorted().collect::<String>();
                    if t.as_str() == two.as_str() {
                        s.push('2');
                    } else if t.as_str() == three.as_str() {
                        s.push('3');
                    } else if t.as_str() == five.as_str() {
                        s.push('5');
                    } else {
                        unreachable!("what happen?")
                    }
                }
                _ => (),
            }
        }
        let n = s.parse::<i32>().unwrap();
        sum += n;
    }
    println!("part 1: {}", count);
    println!("part 2: {}", sum);
}
