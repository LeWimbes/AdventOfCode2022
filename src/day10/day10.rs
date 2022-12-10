fn main() {
    let input = include_str!("day10.txt");
    let instructions: Vec<(i32, i32)> = input.lines().map(|line| {
        if line.starts_with("noop") {
            (1, 0)
        } else {
            let parts = line.split_once(' ').unwrap();
            (2, parts.1.parse().unwrap())
        }
    }).collect();

    task1(&instructions);
    task2(&instructions);
}

fn task1(instructions: &Vec<(i32, i32)>) {
    let mut result = 0;
    let mut cycle = 0;
    let mut x = 1;
    for instruction in instructions {
        for _ in 0..instruction.0 {
            cycle += 1;
            if cycle == 20 || cycle >= 20 && (cycle - 20) % 40 == 0 {
                result += cycle * x;
            }
        }
        x += instruction.1;
    }

    println!("{}", result);
}

fn task2(instructions: &Vec<(i32, i32)>) {
    let mut display: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    for instruction in instructions {
        for _ in 0..instruction.0 {
            if (cycle % 40).abs_diff(x) <= 1 {
                display[(cycle / 40) as usize][(cycle % 40) as usize] = '#';
            }
            cycle += 1;
        }
        x += instruction.1;
    }

    for line in display {
        for cell in line {
            print!("{}", cell);
        }
        println!();
    }
}