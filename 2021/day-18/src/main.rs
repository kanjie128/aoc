use std::cmp::max;

#[derive(Debug, Clone)]
enum Elem {
    N(i32),
    Pair(Node),
}

#[derive(Debug, Clone)]
struct Node {
    left: Box<Elem>,
    right: Box<Elem>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Elem::N(n1), Elem::N(n2)) => {
                write!(f, "[{},{}]", n1, n2)?;
            }
            (Elem::Pair(p), Elem::N(n)) => {
                write!(f, "[")?;
                Node::fmt(p, f)?;
                write!(f, ",{}]", n)?;
            }
            (Elem::N(n), Elem::Pair(p)) => {
                write!(f, "[{},", n)?;
                Node::fmt(p, f)?;
                write!(f, "]")?;
            }
            (Elem::Pair(p1), Elem::Pair(p2)) => {
                write!(f, "[")?;
                Node::fmt(p1, f)?;
                write!(f, ",")?;
                Node::fmt(p2, f)?;
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl Node {
    fn add(p1: Node, p2: Node) -> Node {
        Self {
            left: Box::new(Elem::Pair(p1)),
            right: Box::new(Elem::Pair(p2)),
        }
    }

    fn need_explode(p: &mut Node, mut deep: i32) -> Option<(i32, i32)> {
        deep += 1;
        // println!("deep: {}, nodes:{}", deep, p);
        if deep >= 5 {
            match (p.left.as_ref(), p.right.as_ref()) {
                (Elem::N(n1), Elem::N(n2)) => {
                    // println!("pair:{:?} need explode", p);
                    return Some((*n1, *n2));
                }
                _ => {
                    panic!("wrong pair");
                }
            }
        }
        while let Elem::Pair(ref mut left) = p.left.as_mut() {
            if let Some((n1, n2)) = Self::need_explode(left, deep) {
                if n1 != -1 && n2 == -1 {
                    if deep == 1 {
                        continue;
                    }
                    return Some((n1, -1));
                }

                if n2 != -1 {
                    if n1 != -1 && n2 != -1 {
                        p.left = Box::new(Elem::N(0));
                    }
                    let mut node = p.right.as_mut();
                    loop {
                        match node {
                            Elem::N(n) => {
                                *n += n2;
                                break;
                            }
                            Elem::Pair(p) => {
                                if let Elem::N(n) = p.left.as_mut() {
                                    *n += n2;
                                    break;
                                } else {
                                    node = p.left.as_mut();
                                }
                            }
                        }
                    }
                    if n1 == -1 {
                        continue;
                    }
                    return Some((n1, -1));
                }
            } else {
                break;
            }
        }
        while let Elem::Pair(ref mut right) = p.right.as_mut() {
            if let Some((n1, n2)) = Self::need_explode(right, deep) {
                if n1 == -1 && n2 != -1 {
                    if deep == 1 {
                        continue;
                    }
                    return Some((-1, n2));
                }
                if n1 != -1 {
                    if n1 != -1 && n2 != -1 {
                        p.right = Box::new(Elem::N(0));
                    }
                    let mut node = p.left.as_mut();
                    loop {
                        match node {
                            Elem::N(n) => {
                                *n += n1;
                                break;
                            }
                            Elem::Pair(p) => {
                                if let Elem::N(n) = p.right.as_mut() {
                                    *n += n1;
                                    break;
                                } else {
                                    node = p.right.as_mut();
                                }
                            }
                        }
                    }
                    if n2 == -1 {
                        continue;
                    }
                    return Some((-1, n2));
                }
            } else {
                break;
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        match self.left.as_mut() {
            Elem::N(n) => {
                if *n >= 10 {
                    self.left = Box::new(Elem::Pair(Node {
                        left: Box::new(Elem::N(*n / 2)),
                        right: Box::new(Elem::N(*n - (*n / 2))),
                    }));
                    return true;
                }
            }
            Elem::Pair(p) => {
                if Self::split(p) {
                    return true;
                }
            }
        }
        match self.right.as_mut() {
            Elem::N(n) => {
                if *n >= 10 {
                    self.right = Box::new(Elem::Pair(Node {
                        left: Box::new(Elem::N(*n / 2)),
                        right: Box::new(Elem::N(*n - (*n / 2))),
                    }));
                    return true;
                }
            }
            Elem::Pair(p) => {
                if Self::split(p) {
                    return true;
                }
            }
        }
        false
    }

    fn explode(&mut self) {
        Self::need_explode(self, 0);
    }

    fn magnitude(&self) -> i32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Elem::N(n1), Elem::N(n2)) => 3 * n1 + 2 * n2,
            (Elem::N(n1), Elem::Pair(p)) => 3 * n1 + 2 * Self::magnitude(p),
            (Elem::Pair(p), Elem::N(n2)) => 3 * Self::magnitude(p) + 2 * n2,
            (Elem::Pair(p1), Elem::Pair(p2)) => 3 * Self::magnitude(p1) + 2 * Self::magnitude(p2),
        }
    }
}

fn parse_input(input: &str) -> Result<Node, String> {
    let mut elems = vec![];
    for c in input.trim().chars() {
        match c {
            ',' => {}
            '[' => {}
            ']' => {
                let e2 = elems.pop().unwrap();
                let e1 = elems.pop().unwrap();
                let e = Elem::Pair(Node {
                    left: e1,
                    right: e2,
                });
                elems.push(Box::new(e));
            }
            n => {
                elems.push(Box::new(Elem::N(n as i32 - '0' as i32)));
            }
        }
    }
    if let Some(elem) = elems.pop() {
        if let Elem::Pair(p) = *elem {
            Ok(p)
        } else {
            Err("parse error".to_string())
        }
    } else {
        Err("parse error".to_string())
    }
}

fn part1(mut snailfish: Vec<Node>) -> i32 {
    let mut first = snailfish.remove(0);
    for p in snailfish {
        first = Node::add(first, p);
        first.explode();
        while first.split() {
            first.explode();
        }
    }
    first.magnitude()
}

fn part2(snailfish: Vec<Node>) -> i32 {
    let mut max_sum = 0;
    for (i, p1) in snailfish.iter().enumerate() {
        for (j, p2) in snailfish.iter().enumerate() {
            if i != j {
                let sum = part1(vec![p1.clone(), p2.clone()]);
                max_sum = max(max_sum, sum);
                let sum = part1(vec![p2.clone(), p1.clone()]);
                max_sum = max(max_sum, sum);
            }
        }
    }
    max_sum
}

fn main() {
    let s = include_str!("input");
    let mut snailfish_list = vec![];
    for line in s.lines() {
        let p = parse_input(line).unwrap();
        snailfish_list.push(p);
    }
    let snailfish_list_cp = snailfish_list.clone();
    println!("part1:{}", part1(snailfish_list));
    println!("part2:{}", part2(snailfish_list_cp));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_works() {
        let s = include_str!("sample");
        let mut snailfish_list = vec![];
        for line in s.lines() {
            let p = parse_input(line).unwrap();
            snailfish_list.push(p);
        }
        let snailfish_list_cp = snailfish_list.clone();
        assert_eq!(4140, part1(snailfish_list));
        assert_eq!(3993, part2(snailfish_list_cp));
    }
}
