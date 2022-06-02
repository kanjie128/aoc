struct Image {
    pixels: Vec<Vec<i32>>,
    alg: Vec<i32>,
}

impl Image {
    fn new(pixels: Vec<Vec<i32>>, alg: Vec<i32>) -> Self {
        Self { pixels, alg }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!(
            "---------{}x{}-------------",
            self.pixels.len(),
            self.pixels[0].len()
        );
        for row in &self.pixels {
            for col in row {
                print!("{}", if *col == 0 { "." } else { "#" });
            }
            println!()
        }
        println!("----------------------");
    }

    fn get_alg_pixel(&self, row: i32, col: i32, iter: usize) -> i32 {
        let infinity_pixel = self.get_infinity_pixel(iter);
        let mut alg_index = 0;
        for i in (row - 1)..=(row + 1) {
            for j in (col - 1)..=(col + 1) {
                if i >= 0 && j >= 0 {
                    if let Some(row) = self.pixels.get(i as usize) {
                        if let Some(col) = row.get(j as usize) {
                            alg_index = alg_index << 1 | *col;
                        } else {
                            alg_index = alg_index << 1 | infinity_pixel;
                        }
                    } else {
                        alg_index = alg_index << 1 | infinity_pixel;
                    }
                } else {
                    alg_index = alg_index << 1 | infinity_pixel;
                }
            }
        }
        self.alg[alg_index as usize]
    }

    fn get_infinity_pixel(&self, iter: usize) -> i32 {
        if self.alg[0] == 0 {
            return 0;
        }
        if iter % 2 == 1 {
            self.alg[0]
        } else {
            *self.alg.last().unwrap()
        }
    }

    fn expand(&mut self, iter: usize) {
        let infinity_pixel = self.get_infinity_pixel(iter);
        let mut new_vec = vec![];
        (0..self.pixels[0].len() + 2).for_each(|_| new_vec.push(infinity_pixel));

        for row in &mut self.pixels {
            row.insert(0, infinity_pixel);
            row.push(infinity_pixel);
        }
        self.pixels.insert(0, new_vec.clone());
        self.pixels.push(new_vec);
    }

    fn enhancement(&self, iter: usize) -> Self {
        let mut new_image = vec![];
        for (i, row) in self.pixels.iter().enumerate() {
            let mut new_row = vec![];
            for (j, _) in row.iter().enumerate() {
                let alg = self.get_alg_pixel(i as i32, j as i32, iter);
                new_row.push(alg);
            }
            new_image.push(new_row);
        }
        Self {
            pixels: new_image,
            alg: self.alg.clone(),
        }
    }

    fn count_lit_pixel(&self) -> usize {
        let mut cnt = 0;
        self.pixels.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                if *pixel == 1 {
                    cnt += 1
                }
            })
        });
        cnt
    }
}

fn main() {
    let s = include_str!("input");
    let alg = s.lines().next().unwrap();
    let mut alg_vec = vec![];
    alg.chars().for_each(|c| match c {
        '.' => alg_vec.push(0),
        '#' => alg_vec.push(1),
        c => {
            unreachable!("wrong algorithm {c}");
        }
    });
    // build image
    let mut pixels = vec![];
    for line in s.lines().skip(1).filter(|s| !s.is_empty()) {
        let mut row = vec![];
        line.chars().for_each(|c| match c {
            '.' => row.push(0),
            '#' => row.push(1),
            c => unreachable!("wrong image pixel: {}", c),
        });
        pixels.push(row);
    }
    let mut image = Image::new(pixels, alg_vec);
    (0..2).for_each(|iter| {
        image.expand(iter);
        image = image.enhancement(iter);
    });
    println!("part1: {}", image.count_lit_pixel());
    (2..50).for_each(|iter| {
        image.expand(iter);
        image = image.enhancement(iter);
    });
    println!("part2: {}", image.count_lit_pixel());
}
