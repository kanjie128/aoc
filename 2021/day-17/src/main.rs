fn main() {
    let s = include_str!("input");
    let s = s.split('=').collect::<Vec<&str>>();
    let x = s[1];
    let y = s[2].trim();
    let x = x.split(',').collect::<Vec<_>>();
    let x = x[0].trim();

    let x1 = x.split_terminator("..").collect::<Vec<_>>()[0];
    let x2 = x.split_terminator("..").collect::<Vec<_>>()[1];
    let y1 = y.split_terminator("..").collect::<Vec<_>>()[0];
    let y2 = y.split_terminator("..").collect::<Vec<_>>()[1];

    let x1 = x1.parse::<f64>().unwrap();
    let x2 = x2.parse::<f64>().unwrap();
    let y1 = y1.parse::<f64>().unwrap();
    let y2 = y2.parse::<f64>().unwrap();

    let mut ys = vec![];
    let mut y_maxs = vec![];
    (1..=(-1f64 * y1 - 1.0) as usize).for_each(|y| {
        let yy = calc_max_y(y as i32, y1, y2);
        if let Some(v) = yy {
            ys.push(v.0);
            y_maxs.push(v.1);
        }
    });

    println!("part1: {}", y_maxs.iter().max().unwrap());

    (0..=(-1.0 * y1) as usize).for_each(|y| {
        let y = -(y as i32);
        let yy = calc_max_y(y, y1, y2);
        if let Some(v) = yy {
            ys.push(v.0);
            y_maxs.push(v.1);
        }
    });

    let mut xs = vec![];
    (1..=(x2 as usize)).for_each(|x| {
        let xx = calc_max_x(x as i32, x1, x2);
        if let Some(v) = xx {
            xs.push(v);
        }
    });

    let velocity = calc_possible_velocity(
        &xs,
        &ys,
        (x1 as i32)..(x2 as i32 + 1),
        (y1 as i32)..(y2 as i32 + 1),
    );
    println!("part2: {}", velocity.len());
}

fn calc_possible_velocity(
    xs: &[i32],
    ys: &[i32],
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
) -> Vec<(i32, i32)> {
    let x2 = x_range.clone().last().unwrap();
    let y2 = y_range.clone().next().unwrap();
    let x_range = &x_range;
    let y_range = &y_range;
    let mut vec = vec![];
    for x in xs {
        for y in ys {
            let mut start_x = *x;
            let mut start_y = *y;
            let mut distance_x = 0;
            let mut distance_y = 0;
            loop {
                distance_x += start_x;
                distance_y += start_y;
                if x_range.contains(&distance_x)
                    && y_range.contains(&distance_y)
                    && !vec.contains(&(*x, *y))
                {
                    vec.push((*x, *y));
                }
                if distance_x > x2 || distance_y < y2 {
                    break;
                }
                if start_x > 0 {
                    start_x -= 1;
                }
                start_y -= 1;
            }
        }
    }
    vec
}

fn calc_max_x(x: i32, x1: f64, _x2: f64) -> Option<i32> {
    let _ = unary_root(1.0, -1.0, 2.0 * x1 - x.pow(2) as f64 - x as f64)?.ceil();
    // let _ = unary_root(1.0, -1.0, 2.0 * y2 - y.pow(2) as f64 - y as f64)?;
    Some(x)
}

fn calc_max_y(y: i32, y1: f64, y2: f64) -> Option<(i32, usize)> {
    let _ = unary_root(1.0, -1.0, 2.0 * y1 - y.pow(2) as f64 - y as f64)?.ceil();
    let _ = unary_root(1.0, -1.0, 2.0 * y2 - y.pow(2) as f64 - y as f64)?;
    if y < 0 {
        return Some((y, 0));
    }
    Some((y, (1..=y as usize).sum()))
}

fn unary_root(a: f64, b: f64, c: f64) -> Option<f64> {
    let v = b.powf(2.0) - 4.0 * a * c;
    // println!("a:{a},b:{b},c:{c},v:{v}");
    if v < 0.0 {
        return None;
    }
    Some((-1.0 * b + v.sqrt()) / (2.0 * a))
}
