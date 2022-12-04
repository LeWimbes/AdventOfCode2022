fn main() {
    let input = include_str!("day04.txt");
    let pairs: Vec<((i32, i32), (i32, i32))> = input.lines()
        .map(|line| {
            let parts = line.split_once(',').unwrap();
            let first = parts.0.split_once('-').unwrap();
            let second = parts.1.split_once('-').unwrap();
            ((first.0.parse().unwrap(), first.1.parse().unwrap()),
             (second.0.parse().unwrap(), second.1.parse().unwrap()))
        }).collect();

    task1(&pairs);
    task2(&pairs);
}

fn task1(pairs: &Vec<((i32, i32), (i32, i32))>) {
    let result = pairs.iter().fold(0, |acc, cur|
        acc + if cur.0.0 >= cur.1.0 && cur.0.1 <= cur.1.1 || cur.0.0 <= cur.1.0 && cur.0.1 >= cur.1.1
        { 1 } else { 0 });

    println!("{}", result);
}

fn task2(pairs: &Vec<((i32, i32), (i32, i32))>) {
    let result = pairs.iter().fold(0, |acc, cur|
        acc + if cur.0.0 <= cur.1.1 && cur.1.0 <= cur.0.1
        { 1 } else { 0 });

    println!("{}", result);
}