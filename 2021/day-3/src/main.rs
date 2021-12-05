fn get_zero_one_cnt_at_index<'a>(v: &Vec<&'a str>, index: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut zero = vec![];
    let mut one = vec![];
    for line in v {
        if index >= line.len() {
            panic!("index overflow");
        }
        match line.chars().nth(index).unwrap() {
            '0' => zero.push(*line),
            '1' => one.push(*line),
            _ => unreachable!(),
        }
    }
    (zero, one)
}
fn main() {
    let lines = std::fs::read_to_string("input").unwrap();
    let mut v = vec![];
    let mut line_len = 0;
    for line in lines.lines() {
        v.push(line);
        line_len = line.len();
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut o2 = vec![];
    let mut oc2 = vec![];
    for i in 0..line_len {
        if o2.len() > 0 {
            let (zero, one) = get_zero_one_cnt_at_index(&o2, i);
            if one.len() >= zero.len() {
                o2 = one;
            } else {
                o2 = zero;
            }
        }
        if oc2.len() > 0 {
            let (zero, one) = get_zero_one_cnt_at_index(&oc2, i);
            if zero.len() == 0 {
                oc2 = one;
            } else if one.len() == 0 {
                oc2 = zero;
            } else if one.len() >= zero.len() {
                oc2 = zero;
            } else {
                oc2 = one;
            }
        }
        let (zero, one) = get_zero_one_cnt_at_index(&v, i);
        let mut bit_gamma = 0;
        let mut bit_epsilon = 0;
        if zero.len() > one.len() {
            bit_gamma = 0;
            bit_epsilon = 1;

            if o2.len() == 0 {
                o2 = zero;
                oc2 = one;
            }
        } else {
            bit_gamma = 1;
            bit_epsilon = 0;

            if o2.len() == 0 {
                o2 = one;
                oc2 = zero;
            }
        }
        if gamma > 0 || bit_gamma == 1 {
            gamma = gamma << 1 | bit_gamma;
        }
        if epsilon > 0 || bit_epsilon == 1 {
            epsilon = epsilon << 1 | bit_epsilon;
        }
    }
    println!("gamma: {:b}, {}", gamma, gamma);
    println!("epsilon: {:b}, {}", epsilon, epsilon);
    println!("multi: {}", gamma * epsilon);
    let oxygen = i32::from_str_radix(o2.get(0).unwrap(), 2).unwrap();
    let oc2 = i32::from_str_radix(oc2.get(0).unwrap(), 2).unwrap();
    println!("o2 : {:b} {}", oxygen, oxygen);
    println!("Oc2 : {:b} {}", oc2, oc2);
    println!("multi: {}", oxygen * oc2);
}
