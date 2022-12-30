use std::cell::RefCell;

static INPUT: &str = include_str!("input");

#[derive(Debug, Default, Clone, Copy)]
enum Operator {
    #[default]
    Nop,
    Add,
    Mul,
    Mod,
    Square,
}

impl From<&str> for Operator {
    fn from(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            //"-" => Self::Sub,
            "*" => Self::Mul,
            //"/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Operation {
    operator: Operator,
    value: u64,
}

impl Operation {
    fn new(operator: Operator, value: u64) -> Self {
        Self { operator, value }
    }

    fn eval(&self, value: &u64) -> u64 {
        match self.operator {
            Operator::Nop => *value,
            Operator::Add => value + self.value,
            Operator::Mul => value * self.value,
            Operator::Mod => value % self.value,
            Operator::Square => value * value,
        }
    }
}

#[derive(Debug, Default)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: Operation,
    throw: (u32, u32),
    inspects_items_count: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn round<T>(&mut self, limit: T) -> Vec<(u32, u64)>
    where
        T: Fn(u64) -> u64,
    {
        let mut throw_result: Vec<(u32, u64)> = vec![];
        for item in self.items.iter() {
            self.inspects_items_count += 1;
            let mut op_result = self.op.eval(item);
            op_result = limit(op_result);
            if self.test.eval(&op_result) == 0 {
                // (monkey index, value)
                throw_result.push((self.throw.0, op_result));
            } else {
                throw_result.push((self.throw.1, op_result));
            }
        }
        self.items.clear();
        throw_result
    }
}

#[derive(Debug)]
struct Monkeys(Vec<RefCell<Monkey>>);

impl Monkeys {
    fn new() -> Self {
        Self(vec![])
    }

    fn push(&mut self, monkey: Monkey) {
        self.0.push(RefCell::new(monkey));
    }

    fn parse_input(input: &str) -> Self {
        let mut monkeys = Self::new();
        for line in input.lines() {
            if line.starts_with("Monkey") {
                monkeys.push(Monkey::new());
            } else if line.starts_with("  Starting items: ") {
                let items = line
                    .strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|item| item.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                monkeys.0.last_mut().unwrap().get_mut().items = items;
            } else if line.starts_with("  Operation: new = ") {
                let operation = line
                    .strip_prefix("  Operation: new = ")
                    .unwrap()
                    .split(' ')
                    .collect::<Vec<_>>();
                assert_eq!(operation.len(), 3);
                let op_value = operation.get(2).unwrap();
                let (operator, value) = if *op_value == "old" {
                    (Operator::Square, 0)
                } else {
                    (
                        (*operation.get(1).unwrap()).into(),
                        op_value.parse::<u64>().unwrap(),
                    )
                };
                monkeys.0.last_mut().unwrap().get_mut().op = Operation::new(operator, value);
            } else if line.starts_with("  Test: ") {
                let test = line
                    .strip_prefix("  Test: ")
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                monkeys.0.last_mut().unwrap().get_mut().test = Operation::new(Operator::Mod, test);
            } else if !line.is_empty() {
                let throw_to = line.split(' ').last().unwrap().parse::<u32>().unwrap();
                if line.starts_with("    If true: ") {
                    monkeys.0.last_mut().unwrap().get_mut().throw.0 = throw_to;
                } else if line.starts_with("    If false: ") {
                    monkeys.0.last_mut().unwrap().get_mut().throw.1 = throw_to;
                }
            }
        }
        monkeys
    }

    fn throw(&self, throw: (u32, u64)) {
        self.0
            .get(throw.0 as usize)
            .unwrap()
            .borrow_mut()
            .items
            .push(throw.1);
    }

    fn round<T>(&self, limit: T)
    where
        T: Fn(u64) -> u64,
    {
        for monkey in self.0.iter() {
            let throw_result = monkey.borrow_mut().round(&limit);
            for throw in throw_result {
                self.throw(throw);
            }
        }
    }

    fn monkey_business(&self, active: u32) -> u64 {
        let mut business = 1;
        let mut all_inspects_items_count = self
            .0
            .iter()
            .map(|monkey| monkey.borrow().inspects_items_count)
            .collect::<Vec<_>>();
        all_inspects_items_count.sort_by(|a, b| b.cmp(a));
        all_inspects_items_count
            .get(0..active as usize)
            .unwrap()
            .iter()
            .for_each(|count| business *= *count);
        business
    }

    fn part1(&self) {
        for _ in 0..20 {
            self.round(|x: u64| x / 3);
        }
    }

    fn part2(&self) {
        let mut limit = 1;
        for monkey in self.0.iter() {
            limit *= monkey.borrow().test.value;
        }
        for _ in 0..10000 {
            self.round(|x: u64| x % limit);
        }
    }
}

fn main() {
    let monkeys = Monkeys::parse_input(INPUT);
    monkeys.part1();
    println!("part1: {}", monkeys.monkey_business(2));
    let monkeys = Monkeys::parse_input(INPUT);
    monkeys.part2();
    println!("part2: {}", monkeys.monkey_business(2));
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = include_str!("sample");
    #[test]
    fn test_part1_sample() {
        let monkeys = Monkeys::parse_input(SAMPLE);
        println!("{monkeys:?}");
        monkeys.part1();
        println!("---------------");
        println!("{monkeys:?}");
        println!("---------------");
        assert_eq!(10605, monkeys.monkey_business(2));
    }
    #[test]
    fn test_part2_sample() {
        let monkeys = Monkeys::parse_input(SAMPLE);
        monkeys.part2();
        println!("{monkeys:?}");
        assert_eq!(2713310158, monkeys.monkey_business(2));
    }
}
