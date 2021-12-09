const MAX_TIMER: usize = 9;
fn main() {
    let lines = std::fs::read_to_string("input").unwrap();
    let mut timer_table = [0_usize; MAX_TIMER];
    for n in lines.split(',') {
        let n = n.parse::<usize>().unwrap();
        timer_table[n] += 1;
    }

    for i in 0..256 {
        next_day(&mut timer_table);
        if i == 80 - 1 {
            println!("part 1: {}", count_fish(&timer_table));
        }
    }
    println!("part 2: {}", count_fish(&timer_table));
}

fn count_fish(fish: &[usize]) -> usize {
    let mut sum = 0;
    for n in fish {
        sum += *n;
    }
    sum
}

fn next_day(table: &mut [usize]) {
    let v0 = *table.get(0).unwrap();
    for i in 1..table.len() {
        let d1 = *table.get(i).unwrap();
        let d2 = table.get_mut(i - 1).unwrap();
        *d2 = d1;
    }
    *table.get_mut(6).unwrap() += v0;
    *table.get_mut(8).unwrap() = v0;
}
