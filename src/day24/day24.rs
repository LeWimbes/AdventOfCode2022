use std::cell::RefCell;
use std::collections::{HashSet, LinkedList};
use std::fmt::{Debug, Formatter};
use std::ops::Add;

use crate::Direction::{East, North, South, West};

fn main() {
    let input = include_str!("day24.txt");
    let min = Position { x: 0, y: 0 };
    let mut max = min.clone();
    let mut blizzards = Vec::<(Position, Direction)>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, entry)| {
            let pos = Position { x: x as i32, y: y as i32 };
            max = max.max(&pos);
            match entry {
                '^' => blizzards.push((pos, North)),
                '>' => blizzards.push((pos, East)),
                'v' => blizzards.push((pos, South)),
                '<' => blizzards.push((pos, West)),
                _ => {}
            }
        })
    });
    let start = Position { x: min.x + 1, y: min.y };
    let stop = Position { x: max.x - 1, y: max.y };

    task1(&min, &max, &start, &stop, &blizzards);
    task2(&min, &max, &start, &stop, &blizzards);
}

fn task1(min: &Position, max: &Position, start: &Position, stop: &Position, blizzards: &Vec<(Position, Direction)>) {
    println!("{}", calc_time(min.clone(), max.clone(), start.clone(), stop.clone(), &RefCell::new(blizzards.clone())));
}

fn task2(min: &Position, max: &Position, start: &Position, stop: &Position, blizzards: &Vec<(Position, Direction)>) {
    let blizzards = RefCell::new(blizzards.clone());
    let forth1 = calc_time(min.clone(), max.clone(), start.clone(), stop.clone(), &blizzards);
    let back = calc_time(min.clone(), max.clone(), stop.clone(), start.clone(), &blizzards);
    let forth2 = calc_time(min.clone(), max.clone(), start.clone(), stop.clone(), &blizzards);
    println!("{}", forth1 + back + forth2);
}

fn calc_time(min: Position, max: Position, start: Position, stop: Position, blizzards: &RefCell<Vec<(Position, Direction)>>) -> u32 {
    let b_count = blizzards.borrow().len();

    let mut time: u32 = 0;
    let mut queue = LinkedList::<(Position, u32)>::new();
    queue.push_back((start, time));
    let mut new_positions = HashSet::<Position>::new();
    loop {
        time += 1;

        for i in 0..b_count {
            let blizzard = blizzards.borrow()[i];
            let mut new_pos = blizzard.0 + blizzard.1.get_delta();
            if !new_pos.in_box(&min, &max) {
                match blizzard.1 {
                    North => new_pos.y = max.y - 1,
                    East => new_pos.x = min.x + 1,
                    South => new_pos.y = min.y + 1,
                    West => new_pos.x = max.x - 1,
                }
            }
            blizzards.borrow_mut()[i] = (new_pos, blizzard.1);
        }

        while !queue.is_empty() {
            let (last, _) = queue.pop_front().unwrap();
            for pos in last.next_positions() {
                if pos == stop {
                    return time;
                }
                if pos.in_box(&min, &max) || pos == start {
                    new_positions.insert(pos);
                }
            }
        }
        for (blizzard, _) in blizzards.borrow().iter() {
            new_positions.remove(&blizzard);
        }
        for pos in &new_positions {
            queue.push_back((*pos, time));
        }
        new_positions.clear();
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_delta(&self) -> Position {
        match self {
            North => Position { x: 0, y: -1 },
            East => Position { x: 1, y: 0 },
            South => Position { x: 0, y: 1 },
            West => Position { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn max(&self, other: &Self) -> Self {
        Position { x: self.x.max(other.x), y: self.y.max(other.y) }
    }

    fn in_box(&self, min: &Self, max: &Self) -> bool {
        self.x > min.x && self.x < max.x && self.y > min.y && self.y < max.y
    }

    fn next_positions(&self) -> Vec<Self> {
        vec!(Position { x: self.x, y: self.y }, // same
             Position { x: self.x, y: self.y - 1 }, // north
             Position { x: self.x + 1, y: self.y }, // east
             Position { x: self.x, y: self.y + 1 }, // south
             Position { x: self.x - 1, y: self.y }) // west
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}