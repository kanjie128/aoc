use std::collections::HashMap;

fn main() {
    let s = include_str!("input");

    let mut polymer_template = "".to_string();
    let mut rules: HashMap<String, char> = HashMap::new();

    // build a 26*26 table to store adjoint relation in polymer template
    let mut adjoint_table: Vec<Vec<usize>> = vec![vec![0usize; 26]; 26];

    s.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            polymer_template = line.into();
        } else if !line.is_empty() {
            let rule = line.split(" -> ").collect::<Vec<_>>();
            if rule.len() != 2 {
                panic!("invalid insertion pair: {rule:?}");
            }
            let pair = rule[0];
            let c = rule[1].chars().next().unwrap();
            rules.insert(pair.into(), c);
        }
    });

    let first_char = polymer_template.as_str().chars().next().unwrap();
    let last_char = polymer_template.as_str().chars().last().unwrap();

    (0..polymer_template.len() - 1).for_each(|i| {
        if let Some(s) = polymer_template.get(i..=i + 1) {
            let x = *s.chars().collect::<Vec<_>>().first().unwrap() as usize - 'A' as usize;
            let y = *s.chars().collect::<Vec<_>>().last().unwrap() as usize - 'A' as usize;
            adjoint_table[y][x] += 1;
        }
    });

    for step in 1..=40 {
        // every step we record new adjoint point in map
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        (0..26).for_each(|row| {
            for col in 0..26 {
                if adjoint_table[row][col] > 0 {
                    let k = get_rule(col, row);
                    if let Some(c) = rules.get(&k) {
                        let v = map.entry((col, get_index(*c))).or_insert(0);
                        *v += adjoint_table[row][col];
                        let v = map.entry((get_index(*c), row)).or_insert(0);
                        *v += adjoint_table[row][col];
                        adjoint_table[row][col] = 0;
                    }
                }
            }
        });
        for ((x, y), v) in map {
            // merge back new adjoin relation to table
            adjoint_table[y][x] += v;
        }
        if step == 10 {
            println!(
                "part1:{}",
                subtraction(&adjoint_table, first_char, last_char)
            );
        }
        if step == 40 {
            println!(
                "part2:{}",
                subtraction(&adjoint_table, first_char, last_char)
            );
        }
    }
}

// every single char except the 1st and last in polymer template has been calculated twice
// so we need add 1 for the 1st and last char, then divide them all by 2
fn subtraction(board: &[Vec<usize>], first_char: char, last_char: char) -> usize {
    let mut sum = vec![0usize; 26];
    sum[get_index(first_char)] += 1;
    sum[get_index(last_char)] += 1;

    board.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, col)| {
            sum[x] += *col;
            sum[y] += *col;
        });
    });

    sum.retain(|x| *x > 0);
    sum.sort_unstable();
    (sum.last().unwrap() - sum.first().unwrap()) / 2
}

fn get_index(c: char) -> usize {
    c as usize - 'A' as usize
}

fn get_rule(x: usize, y: usize) -> String {
    let mut s = String::new();
    s.push((x as u8 + b'A') as char);
    s.push((y as u8 + b'A') as char);
    s
}

#[allow(dead_code)]
fn dump(board: &[Vec<usize>]) {
    ('A'..='Z').for_each(|c| print!("{c} "));
    println!();
    let mut sum = 0;
    board.iter().for_each(|row| {
        row.iter().for_each(|col| {
            print!("{col} ");
            sum += *col;
        });
        println!();
    });
    println!("sum:{sum}");
    println!("-----------------------------------------");
}
