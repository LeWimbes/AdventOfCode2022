fn main() {
    let input = include_str!("day01.txt");
    let elves: Vec<i32> = input.split("\n\n")
        .map(|elve| elve.lines().map(|line| line.parse::<i32>().unwrap())
            .fold(0, |acc, cur| acc + cur)).collect();

    task1(&elves);
    task2(&elves);
}

fn task1(elves: &Vec<i32>) {
    println!("{}", elves.iter().max().unwrap());
}

fn task2(elves: &Vec<i32>) {
    let mut sorted: Vec<i32> = elves.clone();
    sorted.sort();
    println!("{}", sorted[sorted.len() - 1] + sorted[sorted.len() - 2] + sorted[sorted.len() - 3]);
}