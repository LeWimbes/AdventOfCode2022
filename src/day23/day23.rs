use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::Direction::{East, North, South, West};

fn main() {
    let input = include_str!("day23.txt");
    let elves: HashSet<Position> = input.lines().enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' { Some(Position { x: x as i32, y: y as i32 }) } else { None }
        })).collect();

    task1(&elves);
    task2(&elves);
}

fn task1(elves: &HashSet<Position>) {
    println!("{}", simulate_rounds(10, elves).ok().unwrap());
}

fn task2(elves: &HashSet<Position>) {
    println!("{}", simulate_rounds(usize::MAX, elves).err().unwrap());
}

fn simulate_rounds(rounds: usize, elves: &HashSet<Position>) -> Result<usize, usize> {
    let mut elves = elves.clone();

    let mut directions = LinkedList::from([North, South, West, East]);
    let proposed = Rc::new(RefCell::new(HashMap::<Position, Position>::with_capacity(elves.len())));
    let mut frequencies = HashMap::<Position, usize>::with_capacity(elves.len());
    for r in 1..=rounds {
        elves.iter().for_each(|elven| {
            let tmp = elven.propose(&elves, &directions);
            if tmp.is_some() {
                proposed.borrow_mut().insert(*elven, tmp.unwrap());
            }
        });

        if proposed.borrow().is_empty() {
            return Err(r);
        }

        proposed.borrow().values().for_each(|pos| {
            *frequencies.entry(*pos).or_default() += 1;
        });

        proposed.borrow().iter().filter(|(_, new)| {
            frequencies[new] == 1
        }).for_each(|(old, new)| {
            elves.remove(old);
            elves.insert(*new);
        });

        let tmp = directions.pop_front().unwrap();
        directions.push_back(tmp);
        proposed.borrow_mut().clear();
        frequencies.clear();
    }

    return Ok(count_ground_tiles(&elves));
}

fn count_ground_tiles(elves: &HashSet<Position>) -> usize {
    let mut result = 0;
    let mut min = Position::MAX;
    let mut max = Position::MIN;

    elves.iter().for_each(|elven| {
        min = min.min(elven);
        max = max.max(elven);
    });

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if !elves.contains(&Position { x, y }) {
                result += 1;
            }
        }
    }

    return result;
}

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const MIN: Self = Position { x: i32::MIN, y: i32::MIN };
    const MAX: Self = Position { x: i32::MAX, y: i32::MAX };

    fn min(&self, other: &Self) -> Self {
        Position { x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    fn max(&self, other: &Self) -> Self {
        Position { x: self.x.max(other.x), y: self.y.max(other.y) }
    }

    fn surrounding(&self) -> Vec<Self> {
        vec!(Position { x: self.x - 1, y: self.y - 1 },
             Position { x: self.x, y: self.y - 1 }, // north
             Position { x: self.x + 1, y: self.y - 1 },
             Position { x: self.x + 1, y: self.y }, // east
             Position { x: self.x + 1, y: self.y + 1 },
             Position { x: self.x, y: self.y + 1 }, // south
             Position { x: self.x - 1, y: self.y + 1 },
             Position { x: self.x - 1, y: self.y }, // west
             Position { x: self.x - 1, y: self.y - 1 })
    }

    fn propose(&self, elves: &HashSet<Self>, directions: &LinkedList<Direction>) -> Option<Self> {
        let neighbors = self.surrounding();
        if neighbors.iter().all(|elven| !elves.contains(elven)) { // no neighbors
            return None;
        }

        let north = &neighbors[0..3];
        let east = &neighbors[2..5];
        let south = &neighbors[4..7];
        let west = &neighbors[6..9];

        for direction in directions {
            let neighbors = match direction {
                North => north,
                South => &south,
                West => &west,
                East => &east,
            };
            if neighbors.iter().all(|elven| !elves.contains(elven)) {
                return Some(neighbors[1]);
            }
        }
        return None;
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}