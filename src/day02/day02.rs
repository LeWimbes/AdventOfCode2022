fn main() {
    let input = include_str!("day02.txt");
    let games: Vec<(u32, u32)> = input.lines().map(|line| {
        let parts = line.split_once(' ').unwrap();
        (parts.0.chars().nth(0).unwrap() as u32 - ('A' as u32), parts.1.chars().nth(0).unwrap() as u32 - ('X' as u32))
    }).collect();
    // Rock 0, Paper 1, Scissors 2
    // Loose 0, Draw 1, Win 2

    task1(&games);
    task2(&games);
}

fn task1(games: &Vec<(u32, u32)>) {
    let mut score = 0;
    for game in games {
        score += game.1 + 1; // own symbol

        if game.0 == game.1 {
            score += 3;
        } else if (game.1 + 2) % 3 == game.0 {
            score += 6;
        }
    }

    println!("{}", score);
}

fn task2(games: &Vec<(u32, u32)>) {
    let mut score = 0;
    for game in games {
        score += game.1 * 3; // loose draw win

        score += match game.1 {
            0 => ((game.0 + 2) % 3) + 1,
            1 => game.0 + 1,
            2 => ((game.0 + 1) % 3) + 1,
            _ => 0
        };
    }

    println!("{}", score);
}