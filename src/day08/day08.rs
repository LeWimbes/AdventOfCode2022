use std::ops::Range;

fn main() {
    let input = include_str!("day08.txt");
    let forest: Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap()).collect()).collect();

    task1(&forest);
    task2(&forest);
}

fn task1(forest: &Vec<Vec<u32>>) {
    let is_on_edge = |i: usize, j: usize| -> bool {
        return i == 0 || j == 0 || i == forest.len() - 1 || j == forest[i].len() - 1;
    };
    let max_in_range = |ri: Range<usize>, rj: Range<usize>| -> u32 {
        let mut max: u32 = 0;
        for i in ri {
            for j in rj.clone() {
                if forest[i][j] > max {
                    max = forest[i][j];
                }
            }
        }
        return max;
    };
    let is_visible = |i: usize, j: usize| -> bool {
        return is_on_edge(i, j)
            || max_in_range(i..i + 1, 0..j) < forest[i][j] // left
            || max_in_range(i..i + 1, j + 1..forest[i].len()) < forest[i][j] // right
            || max_in_range(0..i, j..j + 1) < forest[i][j] // up
            || max_in_range(i + 1..forest.len(), j..j + 1) < forest[i][j]; // down
    };

    let mut count: u32 = 0;
    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            if is_visible(i, j) {
                count += 1;
                continue;
            }
        }
    }

    println!("{}", count);
}

fn task2(forest: &Vec<Vec<u32>>) {
    fn visible_in_range(ri: impl Iterator<Item=usize>, rj: impl Iterator<Item=usize>,
                        height: u32, forest: &Vec<Vec<u32>>) -> u32 {
        let inner: Vec<usize> = rj.collect();
        let mut count: u32 = 0;
        for i in ri {
            for j in inner.iter() {
                count += 1;
                if forest[i][*j] >= height {
                    return count;
                }
            }
        }
        return count;
    }
    let get_score = |i: usize, j: usize| -> u32 {
        return visible_in_range(i..=i, (0..j).rev(), forest[i][j], forest) // left
            * visible_in_range(i..=i, j + 1..forest[i].len(), forest[i][j], forest) // right
            * visible_in_range((0..i).rev(), j..=j, forest[i][j], forest) // up
            * visible_in_range(i + 1..forest.len(), j..=j, forest[i][j], forest); // down
    };

    let mut best_score: u32 = 0;
    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            let score = get_score(i, j);
            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("{}", best_score);
}