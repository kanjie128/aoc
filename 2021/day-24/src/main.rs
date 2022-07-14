use cached::proc_macro::cached;
use cached::Cached;
use cached::SizedCache;

type Ret = (bool, i64);

#[derive(Debug)]
enum OpType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl From<&str> for OpType {
    fn from(s: &str) -> Self {
        match s {
            "inp" => Self::Inp,
            "add" => Self::Add,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "mod" => Self::Mod,
            "eql" => Self::Eql,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Variable {
    W(u8),
    X(u8),
    Y(u8),
    Z(u8),
    N(i64),
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        match s {
            "w" => Self::W(0),
            "x" => Self::X(1),
            "y" => Self::Y(2),
            "z" => Self::Z(3),
            "" => Self::N(0),
            _ => Self::N(s.parse::<i64>().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Op {
    typ: OpType,
    value1: Variable,
    value2: Variable,
}

/// cached provide memoized function
#[cached(
    name = "CACHED_DATA",
    type = "SizedCache<(usize,i64,i64,i64,i64), Ret>",
    create = "{SizedCache::with_size(10000000)}",
    convert = "{(op_index, w, x, y, z)}"
)]
fn resolve(ops: &[Op], range: &[i64], op_index: usize, w: i64, x: i64, y: i64, z: i64) -> Ret {
    if op_index >= ops.len() {
        return (z == 0, 0);
    }
    let mut store = [w, x, y, z];
    let store_get = |s: &Variable, store: &[i64; 4]| -> i64 {
        use Variable::*;
        match s {
            W(n) | X(n) | Y(n) | Z(n) => store[*n as usize],
            N(n) => *n,
        }
    };

    let op = &ops[op_index];
    let v1 = store_get(&op.value1, &store);
    let v2 = store_get(&op.value2, &store);

    let mut store_set = |s: &Variable, v: i64| {
        use Variable::*;
        match s {
            W(n) | X(n) | Y(n) | Z(n) => store[*n as usize] = v,
            _ => unreachable!(),
        }
    };

    match op.typ {
        OpType::Inp => {
            for next_w in range {
                let res = resolve(ops, range, op_index + 1, *next_w, x, y, z);
                if res.0 {
                    return (res.0, next_w + res.1 * 10);
                }
            }
            return (false, 0);
        }
        OpType::Add => {
            store_set(&op.value1, v1 + v2);
        }
        OpType::Mul => {
            store_set(&op.value1, v1 * v2);
        }
        OpType::Div => {
            if v2 == 0 {
                return (false, 0);
            }
            store_set(&op.value1, v1 / v2);
        }
        OpType::Mod => {
            if v1 < 0 || v2 <= 0 {
                return (false, 0);
            }
            store_set(&op.value1, v1 % v2);
        }
        OpType::Eql => {
            store_set(&op.value1, if v1 == v2 { 1 } else { 0 });
        }
    }
    resolve(
        ops,
        range,
        op_index + 1,
        store_get(&("w".into()), &store),
        store_get(&("x".into()), &store),
        store_get(&("y".into()), &store),
        store_get(&("z".into()), &store),
    )
}

fn main() {
    let s = include_str!("input");
    let mut ops = vec![];
    for line in s.lines() {
        let op = line.trim().split(' ').collect::<Vec<_>>();
        ops.push(Op {
            typ: op[0].into(),
            value1: op[1].into(),
            value2: if op[0] == "inp" {
                "".into()
            } else {
                op[2].into()
            },
        });
    }
    let mut digit_range = (1..10).collect::<Vec<i64>>();
    digit_range.reverse();
    println!(
        "part1: {:?}",
        resolve(&ops, &digit_range, 0, 0, 0, 0, 0)
            .1
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
    );

    // clear function cache data for part2
    CACHED_DATA.lock().unwrap().cache_clear();
    digit_range.reverse();
    println!(
        "part2: {:?}",
        resolve(&ops, &digit_range, 0, 0, 0, 0, 0)
            .1
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
    );
}
