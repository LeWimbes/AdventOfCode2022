use std::collections::LinkedList;

fn main() {
    let input = include_str!("day05.txt");
    let parts = input.split_once("\n\n").unwrap();

    let containers_str: Vec<String> = parts.0.lines().map(|line| line.to_string()).collect();
    let stack_count = (containers_str[0].len() + 1) / 4;
    let mut stacks: Vec<LinkedList<char>> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(LinkedList::<char>::new());
    }
    for i in 0..containers_str.len() - 1 {
        let mut chars = containers_str[i].chars();
        for j in 0..stacks.len() {
            let index = if j == 0 { 1 } else { 3 };
            let container = chars.nth(index).unwrap();
            if container != ' ' {
                stacks[j].push_back(container);
            }
        }
    }

    let instructions: Vec<(u32, u32, u32)> = parts.1.lines().map(|line| {
        let mut split = line.split(' ');
        (split.nth(1).unwrap().parse::<u32>().unwrap(),
         split.nth(1).unwrap().parse::<u32>().unwrap() - 1,
         split.nth(1).unwrap().parse::<u32>().unwrap() - 1)
    }).collect();

    task1(&mut stacks.clone(), &instructions);
    task2(&mut stacks.clone(), &instructions);
}

fn task1(stacks: &mut Vec<LinkedList<char>>, instructions: &Vec<(u32, u32, u32)>) {
    for instruction in instructions {
        for _ in 0..instruction.0 {
            let tmp = stacks[instruction.1 as usize].pop_front().unwrap();
            stacks[instruction.2 as usize].push_front(tmp);
        }
    }

    for stack in stacks {
        print!("{}", stack.front().unwrap())
    }
    println!()
}

fn task2(stacks: &mut Vec<LinkedList<char>>, instructions: &Vec<(u32, u32, u32)>) {
    for instruction in instructions {
        let mut tmp_stack = LinkedList::<char>::new();
        for _ in 0..instruction.0 {
            let tmp = stacks[instruction.1 as usize].pop_front().unwrap();
            tmp_stack.push_front(tmp);
        }
        for _ in 0..instruction.0 {
            let tmp = tmp_stack.pop_front().unwrap();
            stacks[instruction.2 as usize].push_front(tmp);
        }
    }

    for stack in stacks {
        print!("{}", stack.front().unwrap())
    }
    println!()
}