use std::collections::HashMap;

use crate::Expr::{Op, Val};
use crate::Operation::{Add, Div, Mul, Sub};

fn main() {
    let input = include_str!("day21.txt");
    let expressions = input.lines()
        .map(|line| {
            let expr = line.split_once(": ").unwrap();

            let tmp = expr.1.parse::<i64>();

            (expr.0, if tmp.is_ok() {
                Val(expr.0, tmp.unwrap())
            } else {
                let parts = expr.1.splitn(3, ' ').collect::<Vec<_>>();
                Op(expr.0, parts[0], parts[2], match parts[1] {
                    "+" => Add,
                    "-" => Sub,
                    "*" => Mul,
                    "/" => Div,
                    _ => panic!("Unknown operator.")
                })
            })
        }
        ).collect::<HashMap<_, _>>();

    task1(&expressions);
    task2(&expressions);
}

fn task1(expressions: &HashMap<&str, Expr>) {
    println!("{}", expressions["root"].evaluate(&expressions));
}

fn task2(expressions: &HashMap<&str, Expr>) {
    let root = &expressions["root"];

    let (_, a, b, _) = root.get_op();
    let a = &expressions[a];
    let b = &expressions[b];

    let tmp = a.contains("humn", expressions);
    let complete = if tmp { b } else { a };
    let missing = if tmp { a } else { b };

    let mut target = complete.evaluate(expressions);
    let mut cur = missing;
    while cur.get_name() != "humn" {
        let (_, a, b, op) = cur.get_op();
        let a = &expressions[a];
        let b = &expressions[b];

        let tmp = a.contains("humn", expressions);
        if tmp {
            let val = b.evaluate(expressions);
            match op {
                Add => target -= val,
                Sub => target += val,
                Mul => target /= val,
                Div => target *= val,
            }
            cur = a;
        } else {
            let val = a.evaluate(expressions);
            match op {
                Add => target -= val,
                Sub => target = val - target,
                Mul => target /= val,
                Div => target = val / target,
            }
            cur = b;
        }
    }

    println!("{}", target);
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn evaluate(&self, a: i64, b: i64) -> i64 {
        return match self {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
        };
    }
}

enum Expr<'a> {
    Val(&'a str, i64),
    Op(&'a str, &'a str, &'a str, Operation),
}

impl Expr<'_> {
    fn get_name(&self) -> &str {
        return match self {
            Val(name, _) => name,
            Op(name, _, _, _) => name,
        };
    }

    fn get_op(&self) -> (&str, &str, &str, &Operation) {
        return match self {
            Val(_, _) => panic!("No operation expression"),
            Op(name, a, b, op) => (name, a, b, op),
        };
    }
    fn evaluate(&self, expressions: &HashMap<&str, Expr>) -> i64 {
        return match self {
            Val(_, val) => *val,
            Op(_, a, b, op) => op.evaluate(expressions[a].evaluate(expressions),
                                           expressions[b].evaluate(expressions)),
        };
    }

    fn contains(&self, expr: &str, expressions: &HashMap<&str, Expr>) -> bool {
        return match self {
            Val(name, _) => *name == expr,
            Op(name, a, b, _) => *name == expr ||
                expressions[a].contains(expr, expressions) ||
                expressions[b].contains(expr, expressions),
        };
    }
}