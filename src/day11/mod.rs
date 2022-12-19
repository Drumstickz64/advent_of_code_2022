use itertools::Itertools;

pub fn solve_part_one(input: String) -> String {
    const NUM_ROUNDS: u64 = 20;
    solve_common(input, NUM_ROUNDS, true)
}

pub fn solve_part_two(input: String) -> String {
    const NUM_ROUNDS: u64 = 10_000;
    solve_common(input, NUM_ROUNDS, false)
}

fn solve_common(input: String, num_rounds: u64, relief: bool) -> String {
    let mut monkeys = parse_input(&input);
    let chinese_remainder = calc_chinese_remainder(&monkeys);
    for _ in 0..num_rounds {
        for monkey_idx in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_idx].items.len() {
                let (item, next_monkey_idx) =
                    monkeys[monkey_idx].inspect_item(relief, chinese_remainder);
                monkeys[next_monkey_idx].items.push(item);
            }
        }
    }

    calc_monkey_business(&monkeys).to_string()
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<WorryLevel>,
    operation: Operation,
    divisible_by: WorryLevel,
    to_throw_if_true: usize,
    to_throw_if_false: usize,
    num_inspections: u64,
}

impl Monkey {
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines().skip(1);
        let starting_items = lines.next().unwrap()[18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect_vec();

        let operation = Operation::from_input(&lines.next().unwrap()[19..]);
        let divisible_by = lines.next().unwrap()[21..].parse().unwrap();

        let to_throw_if_true = lines.next().unwrap()[29..].parse().unwrap();
        let to_throw_if_false = lines.next().unwrap()[30..].parse().unwrap();

        Self {
            items: starting_items,
            operation,
            divisible_by,
            to_throw_if_true,
            to_throw_if_false,
            num_inspections: 0,
        }
    }

    pub fn inspect_item(&mut self, relief: bool, chinese_remainder: u64) -> (WorryLevel, usize) {
        let item = self.items.remove(0);
        let item = self.operation.calc_new_priority(item);
        let item = if relief { item / 3 } else { item };
        let item = item % chinese_remainder;
        self.num_inspections += 1;
        let next_idx = if item % self.divisible_by == 0 {
            self.to_throw_if_true
        } else {
            self.to_throw_if_false
        };
        (item, next_idx)
    }
}

type WorryLevel = u64;

#[derive(Debug)]
struct Operation {
    operation_type: OperationType,
    rhs: Value,
}

impl Operation {
    pub fn from_input(input: &str) -> Self {
        let mut parts = input.split_ascii_whitespace().skip(1);
        let operation_type = OperationType::from_input(parts.next().unwrap());
        let rhs = Value::from_input(parts.next().unwrap());
        Self {
            operation_type,
            rhs,
        }
    }

    pub fn calc_new_priority(&self, item: WorryLevel) -> WorryLevel {
        let rhs = match self.rhs {
            Value::Old => item,
            Value::New(new) => new,
        };

        match self.operation_type {
            OperationType::Add => item + rhs,
            OperationType::Mul => item * rhs,
        }
    }
}

#[derive(Debug)]
enum OperationType {
    Add,
    Mul,
}

impl OperationType {
    pub fn from_input(input: &str) -> Self {
        match input {
            "+" => OperationType::Add,
            "*" => OperationType::Mul,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Value {
    Old,
    New(WorryLevel),
}

impl Value {
    pub fn from_input(input: &str) -> Self {
        match input {
            "old" => Value::Old,
            num => Value::New(num.parse().unwrap()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from_input).collect()
}

fn calc_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut activity = monkeys
        .iter()
        .map(|monkey| monkey.num_inspections)
        .collect_vec();
    activity.sort_unstable();
    activity[activity.len() - 2..activity.len()]
        .iter()
        .product()
}

fn calc_chinese_remainder(monkeys: &[Monkey]) -> u64 {
    monkeys.iter().map(|monkey| monkey.divisible_by).product()
}
