use std::ops::{Add, Mul};
use regex::Regex;
use crate::utils::get_input;

pub fn part1() -> usize {
    execute(20, 3)
}

pub fn part2() -> usize {
    execute(10_000, 1)
}

pub fn execute(num_rounds: usize, divisor: i64) -> usize {
    let mut monkeys = get_monkeys();
    let lcd: i64 = monkeys.iter().map(|m| m.divisible_by_test).product();

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            monkeys[i].inspection_count += monkeys[i].items.len();

            for item in monkeys[i].items.clone() {
                let item = monkeys[i].get_new_worry_level(item) / divisor;
                let new_index = if (item % monkeys[i].divisible_by_test) == 0 {
                    monkeys[i].monkey_if_true
                } else {
                    monkeys[i].monkey_if_false
                };
                
                monkeys[new_index].items.push(item % lcd);
            }

            monkeys[i].items.clear();
        }
    }

    let mut inspection_counts = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    lhs: Arg,
    rhs: Arg,
    op: fn(i64, i64) -> i64,
    divisible_by_test: i64,
    monkey_if_true: usize,
    monkey_if_false: usize,
    inspection_count: usize,
}

impl Monkey {
    fn get_new_worry_level(&self, old: i64) -> i64 {
        let lhs = match self.lhs {
            Arg::Const(n) => n,
            Arg::Old => old,
        };

        let rhs = match self.rhs {
            Arg::Const(n) => n,
            Arg::Old => old,
        };

        (self.op)(lhs, rhs)
    }
}

#[derive(Debug, Clone)]
enum Arg {
    Old,
    Const(i64),
}

fn get_monkeys() -> Vec<Monkey> {
    let mut monkeys = vec![];

    let regex = Regex::new(
r"Monkey \d+:
  Starting items: (?P<items>[\d, ]+)
  Operation: new = (?P<lhs>(\w|\d)+) (?P<op>.) (?P<rhs>(\w|\d)+)
  Test: divisible by (?P<divisible>\d+)
    If true: throw to monkey (?P<if_true>\d+)
    If false: throw to monkey (?P<if_false>\d+)").unwrap();
    
    for capture in regex.captures_iter(&get_input(11)[..]) {
        let lhs = match capture.name("lhs").unwrap().as_str() {
            "old" => Arg::Old,
            num => Arg::Const(num.parse().unwrap()),
        };

        let rhs = match capture.name("rhs").unwrap().as_str() {
            "old" => Arg::Old,
            num => Arg::Const(num.parse().unwrap()),
        };
        
        let op = match capture.name("op").unwrap().as_str() {
            "+" => i64::add,
            "*" => i64::mul,
            _ => unreachable!(),
        };

        monkeys.push(Monkey {
            items: capture.name("items").unwrap().as_str().split(", ").map(|s| s.parse().unwrap()).collect(),
            lhs,
            rhs,
            op,
            divisible_by_test: capture.name("divisible").unwrap().as_str().parse().unwrap(),
            monkey_if_true: capture.name("if_true").unwrap().as_str().parse().unwrap(),
            monkey_if_false: capture.name("if_false").unwrap().as_str().parse().unwrap(),
            inspection_count: 0,
        });
    }

    monkeys
}
