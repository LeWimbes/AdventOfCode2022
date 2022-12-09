use std::collections::HashSet;

fn main() {
    let input = include_str!("day09.txt");
    let instructions: Vec<Instruction> = input.lines().map(|line| {
        let parts = line.split_once(' ').unwrap();
        Instruction {
            dir: match parts.0 {
                "R" => (1, 0),
                "L" => (-1, 0),
                "D" => (0, 1),
                "U" => (0, -1),
                _ => (0, 0)
            },
            step: parts.1.parse().unwrap(),
        }
    }).collect();

    task1(&instructions);
    task2(&instructions);
}

fn task1(instructions: &Vec<Instruction>) {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(tail);

    for instruction in instructions {
        for _ in 0..instruction.step {
            head = add(head, instruction.dir);
            tail = update_tail(head, tail);
            positions.insert(tail);
        }
    }

    println!("{}", positions.len());
}

fn task2(instructions: &Vec<Instruction>) {
    let mut knots = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
                         (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(knots[9]);

    for instruction in instructions {
        for _ in 0..instruction.step {
            knots[0] = add(knots[0], instruction.dir);
            for i in 0..9 {
                knots[i + 1] = update_tail(knots[i], knots[i + 1]);
            }
            positions.insert(knots[9]);
        }
    }

    println!("{}", positions.len());
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn update_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 { // close enough
        tail
    } else if head.0.abs_diff(tail.0) == 2 && head.1.abs_diff(tail.1) == 2 {
        (if head.0 < tail.0 { head.0 + 1 } else { head.0 - 1 },
         if head.1 < tail.1 { head.1 + 1 } else { head.1 - 1 })
    } else if head.0 == tail.0 || head.1.abs_diff(tail.1) == 2 {
        (head.0, if head.1 > tail.1 { head.1 - 1 } else { head.1 + 1 })
    } else {
        (if head.0 > tail.0 { head.0 - 1 } else { head.0 + 1 }, head.1)
    }
}

struct Instruction {
    dir: (i32, i32),
    step: u32,
}