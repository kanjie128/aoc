use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    // col
    x: i32,
    // row
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Edge {
    weight: i32,
    from: Point,
    to: Point,
}

impl Edge {
    fn new(weight: i32, from: Point, to: Point) -> Self {
        Self { weight, from, to }
    }
}
struct Node {
    _coordinate: Point,
    edges: Vec<Edge>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.edges {
            writeln!(f, "{:?} -> {:?}, {}", e.from, e.to, e.weight)?;
        }
        Ok(())
    }
}

impl Node {
    fn new(x: i32, y: i32, edges: Vec<Edge>) -> Self {
        Self {
            _coordinate: Point::new(x, y),
            edges,
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
}

fn get_input_point(input: &[Vec<i32>], Point { x, y }: Point) -> Option<i32> {
    if y < 0 || x < 0 {
        return None;
    }

    if y as usize > input.len() - 1 {
        return None;
    }

    if x as usize > input.get(y as usize).unwrap().len() - 1 {
        return None;
    }

    Some(*input.get(y as usize).unwrap().get(x as usize).unwrap())
}

impl Graph {
    fn new(input: &[Vec<i32>]) -> Self {
        let mut graph = Self { nodes: vec![] };
        for (y, row) in input.iter().enumerate() {
            let mut row_nodes = vec![];
            for (x, _col) in row.iter().enumerate() {
                let curr_point = Point::new(x as i32, y as i32);
                let mut edges = vec![];
                // up
                let up = Point::new(x as i32, y as i32 - 1);
                if let Some(weight) = get_input_point(input, up) {
                    edges.push(Edge::new(weight, curr_point, up));
                }
                // down
                let down = Point::new(x as i32, y as i32 + 1);
                if let Some(weight) = get_input_point(input, down) {
                    edges.push(Edge::new(weight, curr_point, down));
                }
                // left
                let left = Point::new(x as i32 - 1, y as i32);
                if let Some(weight) = get_input_point(input, left) {
                    edges.push(Edge::new(weight, curr_point, left));
                }
                // right
                let right = Point::new(x as i32 + 1, y as i32);
                if let Some(weight) = get_input_point(input, right) {
                    edges.push(Edge::new(weight, curr_point, right));
                }
                let node = Node::new(x as i32, y as i32, edges);
                row_nodes.push(node);
            }
            graph.nodes.push(row_nodes);
        }
        graph
    }

    fn get_point_node(&self, p: Point) -> &Node {
        self.nodes
            .get(p.y as usize)
            .unwrap()
            .get(p.x as usize)
            .unwrap()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for row in &self.nodes {
            for node in row {
                println!("{:?}", node);
            }
        }
    }
}

#[derive(PartialEq, Eq)]
struct Distance {
    p: Point,
    v: Reverse<i32>,
}

impl Distance {
    fn new(p: Point, v: i32) -> Self {
        Self { p, v: Reverse(v) }
    }
}

impl std::cmp::PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.v.cmp(&other.v)
    }
}

fn main() {
    let s = include_str!("input");
    let mut input = vec![];
    for line in s.lines() {
        let mut row = vec![];
        line.chars().for_each(|c| {
            row.push(c as i32 - '0' as i32);
        });
        input.push(row);
    }
    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<i32>]) {
    let graph = Graph::new(input);
    let row_len = graph.nodes.len();
    let col_len = graph.nodes[0].len();
    let answer = dijkstra(&graph);

    println!(
        "part1: {:?}",
        answer.get(&Point::new(col_len as i32 - 1, row_len as i32 - 1))
    );
}

fn part2(input: &[Vec<i32>]) {
    let mut input_extend = vec![];
    // extend col
    for row in input {
        let mut tmp = vec![];
        (0..5).for_each(|i| {
            for col in row {
                let v = (*col + i) / 10 + (*col + i) % 10;
                tmp.push(v);
            }
        });
        input_extend.push(tmp);
    }
    // dump_input(&input_extend);
    // extend row
    let mut extend_row = vec![];
    (1..5).for_each(|i| {
        for row in &input_extend {
            let mut tmp = vec![];
            for col in row {
                let v = (*col + i) / 10 + (*col + i) % 10;
                tmp.push(v);
            }
            extend_row.push(tmp);
        }
    });
    input_extend.append(&mut extend_row);
    let graph = Graph::new(&input_extend);
    let row_len = graph.nodes.len();
    let col_len = graph.nodes[0].len();
    let answer = dijkstra(&graph);

    println!(
        "part2: {:?}",
        answer.get(&Point::new(col_len as i32 - 1, row_len as i32 - 1))
    );
}

fn dijkstra(graph: &Graph) -> HashMap<Point, i32> {
    let mut answer: HashMap<Point, i32> = HashMap::new();
    let mut open = BinaryHeap::new();
    let mut close = HashMap::new();
    open.push(Distance::new(Point::new(0, 0), 0));
    while let Some(small) = open.pop() {
        if close.get(&small.p).is_some() {
            continue;
        }
        let node = graph.get_point_node(small.p);
        for e in &node.edges {
            let from_weight = if let Some(v) = answer.get(&e.from) {
                *v
            } else {
                0
            };
            match answer.get_mut(&e.to) {
                None => {
                    answer.insert(e.to, e.weight + from_weight);
                }
                Some(ans) => {
                    if *ans > e.weight + from_weight {
                        *ans = e.weight + from_weight;
                    }
                }
            }
            if close.get(&e.to).is_none() {
                open.push(Distance::new(e.to, *answer.get(&e.to).unwrap()));
            }
        }
        close.insert(small.p, 0);
    }
    answer
}

#[allow(dead_code)]
fn dump_input(input: &[Vec<i32>]) {
    input.iter().for_each(|row| {
        row.iter().for_each(|col| {
            print!("{col}");
        });
        println!();
    });
    println!();
}
