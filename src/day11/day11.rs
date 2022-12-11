use std::cell::RefCell;
use std::collections::LinkedList;

fn main() {
    let input = include_str!("day11.txt");
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|monkey| {
        let mut lines = monkey.lines();
        lines.next(); // Monkey x:

        let items_line = lines.next().unwrap();
        let items: LinkedList<u64> = items_line["  Starting items: ".len()..items_line.len()].split(", ").map(|item| item.parse().unwrap()).collect();

        let operation_line = lines.next().unwrap();
        let operation_parts: Vec<&str> = operation_line["  Operation: new = ".len()..operation_line.len()].split(' ').collect();

        let test_line = lines.next().unwrap();
        let divider: u64 = test_line["  Test: divisible by ".len()..test_line.len()].parse().unwrap();
        let true_line = lines.next().unwrap();
        let true_monkey: usize = true_line["    If true: throw to monkey ".len()..true_line.len()].parse().unwrap();
        let false_line = lines.next().unwrap();
        let false_monkey: usize = false_line["    If false: throw to monkey ".len()..false_line.len()].parse().unwrap();

        Monkey {
            start: RefCell::new(items.clone()),
            items: RefCell::new(items.clone()),
            op: if operation_parts[2] == "old" {
                Op::Squ
            } else if operation_parts[1] == "+" {
                Op::Add(operation_parts[2].parse().unwrap())
            } else {
                Op::Mul(operation_parts[2].parse().unwrap())
            },
            test: divider,
            next: (true_monkey, false_monkey),
        }
    }).collect();

    task1(&monkeys);
    restore_monkeys(&monkeys);
    task2(&monkeys);
}

fn task1(monkeys: &Vec<Monkey>) {
    println!("{}", determine_monkey_business(monkeys, 20, |worry| worry / 3));
}

fn task2(monkeys: &Vec<Monkey>) {
    let m = monkeys.iter().fold(1, |acc, cur| { acc * cur.test });
    println!("{}", determine_monkey_business(monkeys, 10000, |worry| worry % m));
}

fn determine_monkey_business(monkeys: &Vec<Monkey>, rounds: usize, worry_modifier: impl Fn(u64) -> u64) -> usize {
    let mut inspections = vec![0 as usize; monkeys.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            while !monkey.items.borrow().is_empty() {
                let mut item = monkey.items.borrow_mut().pop_front().unwrap();
                inspections[i] += 1;

                item = match monkey.op {
                    Op::Add(b) => item + b,
                    Op::Mul(b) => item * b,
                    Op::Squ => item * item,
                };

                item = worry_modifier(item);

                let next = if item % monkey.test == 0 { monkey.next.0 } else { monkey.next.1 };
                monkeys[next].items.borrow_mut().push_back(item);
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn restore_monkeys(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        monkey.items.borrow_mut().clear();
        for item in monkey.start.borrow().iter() {
            monkey.items.borrow_mut().push_back(*item);
        }
    }
}

enum Op {
    Add(u64),
    Mul(u64),
    Squ,
}

struct Monkey {
    start: RefCell<LinkedList<u64>>,
    items: RefCell<LinkedList<u64>>,
    op: Op,
    test: u64,
    next: (usize, usize),
}