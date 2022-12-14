use std::fmt::{Debug, Formatter};

fn main() {
    let input = include_str!("day14.txt");

    let mut start = Coordinate { x: 500, y: 0 };
    let mut min = Coordinate { x: i32::MAX, y: 0 };
    let mut max = Coordinate { x: i32::MIN, y: i32::MIN };
    let mut paths: Vec<Vec<Coordinate>> = input.lines()
        .map(|line| line.split(" -> ")
            .map(|coord_str| {
                let parts = coord_str.split_once(',').unwrap();
                let coord = Coordinate {
                    x: parts.0.parse().unwrap(),
                    y: parts.1.parse().unwrap(),
                };
                min = min.min(&coord);
                max = max.max(&coord);
                coord
            }).collect()).collect();


    let mut grid_min = min.clone();
    let mut grid_max = max.clone();
    grid_max.y += 2;
    grid_min.x = grid_min.x.min(start.x - grid_max.y);
    grid_max.x = grid_max.x.max(start.x + grid_max.y);

    // bottom line
    paths.push(vec![Coordinate { x: grid_min.x, y: grid_max.y }, Coordinate { x: grid_max.x, y: grid_max.y }]);

    let x_shift = grid_min.x;
    start.x -= x_shift;
    min.x -= x_shift;
    max.x -= x_shift;
    grid_min.x -= x_shift;
    grid_max.x -= x_shift;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; (grid_max.x + 1) as usize]; (grid_max.y + 1) as usize];
    grid[start.y as usize][start.x as usize] = '+';
    for path in paths {
        for i in 0..path.len() - 1 {
            let mut cur = path[i];
            let mut next = path[i + 1];
            // shift
            cur.x -= x_shift;
            next.x -= x_shift;
            let tmp = cur.min(&next);
            let next = cur.max(&next);
            cur = tmp;

            for j in cur.x..=next.x {
                for k in cur.y..=next.y {
                    grid[k as usize][j as usize] = '#';
                }
            }
        }
    }
    task1(&mut grid, &start, &min, &max);
    restore_grid(&mut grid);
    task2(&mut grid, &start, &grid_min, &grid_max);
    restore_grid(&mut grid);
}

fn task1(grid: &mut Vec<Vec<char>>, start: &Coordinate, min: &Coordinate, max: &Coordinate) {
    let mut sand_count: u32 = 0;
    let mut in_grid = true;
    while in_grid {
        let mut sand = start.clone();
        let mut next = sand.get_next(&grid, &min, &max);
        while in_grid && next != sand {
            if next.in_rect(&min, &max) {
                sand = next;
                next = sand.get_next(&grid, &min, &max);
            } else {
                in_grid = false;
            }
        }
        grid[sand.y as usize][sand.x as usize] = 'o';

        sand_count += 1;
    }

    println!("{}", sand_count - 1);
}

fn task2(grid: &mut Vec<Vec<char>>, start: &Coordinate, min: &Coordinate, max: &Coordinate) {
    let mut sand_count: u32 = 0;
    let mut on_start = false;
    while !on_start {
        let mut sand = start.clone();
        let mut next = sand.get_next(&grid, &min, &max);
        while !on_start && next != sand {
            sand = next;
            next = sand.get_next(&grid, &min, &max);
        }
        if next == *start {
            on_start = true;
        } else {
            grid[sand.y as usize][sand.x as usize] = 'o';
        }

        sand_count += 1;
    }

    println!("{}", sand_count);
}

fn restore_grid(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'o' {
                grid[i][j] = '.';
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn min(&self, other: &Self) -> Self {
        Coordinate {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Coordinate {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn get_next(&self, grid: &Vec<Vec<char>>, min: &Coordinate, max: &Coordinate) -> Self {
        // straight down
        let down = Coordinate { x: self.x, y: self.y + 1 };
        if !down.in_rect(min, max) || grid[down.y as usize][down.x as usize] == '.' {
            return down;
        }
        // diagonally left
        let left = Coordinate { x: self.x - 1, y: self.y + 1 };
        if !left.in_rect(min, max) || grid[left.y as usize][left.x as usize] == '.' {
            return left;
        }
        // diagonally right
        let right = Coordinate { x: self.x + 1, y: self.y + 1 };
        if !right.in_rect(min, max) || grid[right.y as usize][right.x as usize] == '.' {
            return right;
        }
        // no free space
        return self.clone();
    }

    fn in_rect(&self, min: &Self, max: &Self) -> bool {
        *min == self.min(min) && *max == self.max(max)
    }
}

impl PartialEq<Self> for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}