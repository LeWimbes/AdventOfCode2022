use std::collections::HashSet;

fn main() {
    let input: Vec<char> = include_str!("day06.txt").chars().collect();

    task1(&input);
    task2(&input);
}

fn task1(input: &Vec<char>) {
    let mut container = HashSet::<char>::new();
    for i in 0..input.len() - 3 {
        let mut unique = true;
        for j in i..i + 4 {
            if !container.insert(input[j]) {
                unique = false;
                break;
            }
        }

        if unique {
            println!("{}", i + 4);
            return;
        } else {
            container.clear();
        }
    }
}

fn task2(input: &Vec<char>) {
    let mut container = HashSet::<char>::new();
    for i in 0..input.len() - 13 {
        let mut unique = true;
        for j in i..i + 14 {
            if !container.insert(input[j]) {
                unique = false;
                break;
            }
        }

        if unique {
            println!("{}", i + 14);
            return;
        } else {
            container.clear();
        }
    }
}