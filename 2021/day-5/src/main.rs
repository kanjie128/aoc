const N: usize = 1000;
const M: usize = 1000;

struct Map {
    map: [[i32; N]; M],
}

impl Map {
    fn new() -> Self {
        Self { map: [[0; N]; M] }
    }
    #[allow(dead_code)]
    fn dump_map(&self) {
        for row in &self.map {
            for n in row {
                if *n == 0 {
                    print!(". ");
                } else {
                    print!("{} ", *n);
                }
            }
            println!();
        }
    }

    fn set_point(&mut self, point: (i32, i32)) {
        *self
            .map
            .get_mut(point.1 as usize)
            .unwrap()
            .get_mut(point.0 as usize)
            .unwrap() += 1;
    }

    fn cover_point(&mut self, start: (i32, i32), end: (i32, i32)) {
        let step;
        if start.0 == end.0 || start.1 == end.1 {
            step = if start.0 < end.0 {
                (1, 0)
            } else if start.0 > end.0 {
                (-1, 0)
            } else if start.1 < end.1 {
                (0, 1)
            } else if start.1 > end.1 {
                (0, -1)
            } else {
                (0, 0)
            };
        } else if (start.0 - end.0).abs() == (start.1 - end.1).abs() {
            step = if start.0 < end.0 && start.1 < end.1 {
                (1, 1)
            } else if start.0 > end.0 && start.1 > end.1 {
                (-1, -1)
            } else if start.0 < end.0 && start.1 > end.1 {
                (1, -1)
            } else {
                (-1, 1)
            };
        } else {
            return;
        }
        let mut start = start;
        while start.0 != end.0 || start.1 != end.1 {
            self.set_point(start);
            start.0 += step.0;
            start.1 += step.1;
        }
        assert!(start.0 == end.0);
        assert!(start.1 == end.1);
        self.set_point(end);
    }

    fn ans(&self) -> i32 {
        let mut ans = 0;
        for row in &self.map {
            for n in row {
                if *n > 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
fn main() {
    let mut map = Map::new();
    let lines = include_str!("input");
    let mut part_1 = vec![];
    let mut part_2 = vec![];
    for line in lines.lines() {
        let point = line
            .split(|c| c == ' ' || c == '-' || c == '>')
            .filter(|s| *s != "")
            .collect::<Vec<_>>();
        assert_eq!(point.len(), 2);

        let start = point.get(0).unwrap();
        let end = point.get(1).unwrap();
        let start = start.split(',').collect::<Vec<_>>();
        assert_eq!(start.len(), 2);
        let end = end.split(',').collect::<Vec<_>>();
        assert_eq!(end.len(), 2);
        let start_x = start.get(0).unwrap().parse::<i32>().unwrap();
        let start_y = start.get(1).unwrap().parse::<i32>().unwrap();
        let end_x = end.get(0).unwrap().parse::<i32>().unwrap();
        let end_y = end.get(1).unwrap().parse::<i32>().unwrap();
        if start_x == end_x || start_y == end_y {
            part_1.push(((start_x, start_y), (end_x, end_y)));
        } else {
            part_2.push(((start_x, start_y), (end_x, end_y)));
        }
    }
    for (start, end) in part_1 {
        map.cover_point(start, end)
    }
    println!("part 1: {}", map.ans());
    for (start, end) in part_2 {
        map.cover_point(start, end)
    }
    println!("part 2: {}", map.ans());
}
