fn main() {
    let input = include_str!("day03.txt");
    let backpacks: Vec<Vec<u32>> = input.lines().
        map(|line| line.chars()
            .map(|c| {
                if c.is_lowercase() {
                    c as u32 - 'a' as u32
                } else {
                    c as u32 - 'A' as u32 + 26
                }
            }).collect()).collect();

    task1(&backpacks);
    task2(&backpacks);
}

fn task1(backpacks: &Vec<Vec<u32>>) {
    let mut result = 0;
    for backpack in backpacks {
        let mut arr = [false; 52];
        for i in 0..(backpack.len() / 2) {
            arr[backpack[i] as usize] = true;
        }
        for i in (backpack.len() / 2)..backpack.len() {
            if arr[backpack[i] as usize] {
                result += backpack[i] + 1;
                break;
            }
        }
    }

    println!("{}", result);
}

fn task2(backpacks: &Vec<Vec<u32>>) {
    let mut result = 0;
    for i in (0..backpacks.len()).step_by(3) {
        let mut arr = [0; 52];
        for j in 0..backpacks[i].len() {
            let index = backpacks[i][j] as usize;
            if arr[index] == 0 {
                arr[index] = 1;
            }
        }
        for j in 0..backpacks[i + 1].len() {
            let index = backpacks[i + 1][j] as usize;
            if arr[index] == 1 {
                arr[index] = 2;
            }
        }
        for j in 0..backpacks[i + 2].len() {
            if arr[backpacks[i + 2][j] as usize] == 2 {
                result += backpacks[i + 2][j] + 1;
                break;
            }
        }
    }

    println!("{}", result);
}