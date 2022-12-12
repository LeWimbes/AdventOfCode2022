use std::collections::VecDeque;

fn main() {
    let input = include_str!("day12.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); height * width];
    let mut start: usize = usize::MAX;
    let mut target: usize = usize::MAX;

    for i in 0..height {
        for j in 0..width {
            let index = i * width + j;
            if grid[i][j] == 'S' {
                start = index;
                grid[i][j] = 'a';
                if target != usize::MAX {
                    break;
                }
            } else if grid[i][j] == 'E' {
                target = index;
                grid[i][j] = 'z';
                if start != usize::MAX {
                    break;
                }
            }
        }
        if start != usize::MAX && target != usize::MAX {
            break;
        }
    }

    for i in 0..height {
        for j in 0..width {
            let elevation = grid[i][j];
            let index = i * width + j;

            if i > 0 && grid[i - 1][j] as i32 - elevation as i32 <= 1 {
                adj[index].push(index - width);
            }
            if i < height - 1 && grid[i + 1][j] as i32 - elevation as i32 <= 1 {
                adj[index].push(index + width);
            }
            if j > 0 && grid[i][j - 1] as i32 - elevation as i32 <= 1 {
                adj[index].push(index - 1);
            }
            if j < width - 1 && grid[i][j + 1] as i32 - elevation as i32 <= 1 {
                adj[index].push(index + 1);
            }
        }
    }

    task1(&adj, height, width, start, target);
    task2(&adj, &grid, height, width, target);
}

fn task1(adj: &Vec<Vec<usize>>, height: usize, width: usize, start: usize, target: usize) {
    println!("{}", breadth_first_search(adj, height, width, start, target))
}

fn task2(adj: &Vec<Vec<usize>>, grid: &Vec<Vec<char>>, height: usize, width: usize, target: usize) {
    let starts: Vec<usize> = grid.iter().enumerate()
        .map(|row| row.1.iter().enumerate()
            .filter_map(|entry| {
                if *entry.1 == 'a' {
                    Some(row.0 * width + entry.0)
                } else {
                    None
                }
            }).collect::<Vec<usize>>()).flatten().collect();

    let mut min = u32::MAX;
    for start in starts {
        let path = breadth_first_search(adj, height, width, start, target);
        if path < min {
            min = path;
        }
    }

    println!("{}", min)
}

fn breadth_first_search(adj: &Vec<Vec<usize>>, height: usize, width: usize, start: usize, target: usize) -> u32 {
    let mut queue = VecDeque::<usize>::new();
    let mut distances = vec![u32::MAX; height * width];

    queue.push_back(start);
    distances[start] = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for child in &adj[current] {
            if distances[*child] != u32::MAX {
                continue;
            }
            distances[*child] = distances[current] + 1;
            queue.push_back(*child);
        }
    }

    distances[target]
}