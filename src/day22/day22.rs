use std::collections::HashMap;
use std::ops::Add;

use regex::Regex;

use crate::Direction::{East, North, South, West};
use crate::Instruction::{Rotate, Step};
use crate::Rotation::{Left, Right};

fn main() {
    let input = include_str!("day22.txt");
    let board_path = input.split_once("\n\n").unwrap();

    let mut board: Vec<Vec<char>> = board_path.0.lines().map(|line| line.chars().collect()).collect();
    let size = Position {
        x: board.iter().fold(0, |max, cur| max.max(cur.len())) as i32,
        y: board.len() as i32,
    };
    // fill rectangle
    board.iter_mut().for_each(|row| row.resize(size.x as usize, ' '));

    let rx: Regex = Regex::new(r"[LR]|\d+").unwrap();
    let path = rx.captures_iter(board_path.1).map(|capture| {
        if capture[0] == *"L" {
            Rotate(Left)
        } else if capture[0] == *"R" {
            Rotate(Right)
        } else {
            Step(capture[0].parse().unwrap())
        }
    }).collect::<Vec<Instruction>>();

    task1(&mut board, &path);
    task2(&mut board, &path);
}

fn task1(board: &mut Vec<Vec<char>>, path: &Vec<Instruction>) {
    let mut crossovers = HashMap::<State, State>::new();

    let test_data = false; // unfortunately without automatic detection :/
    if test_data { // for test data
        for x in 8..12 { // 0 to 6
            crossovers.insert(State { pos: Position { x, y: -1 }, dir: North },
                              State { pos: Position { x, y: 11 }, dir: North });
        }
        for y in 0..4 { // 1 to 13
            crossovers.insert(State { pos: Position { x: 12, y }, dir: East },
                              State { pos: Position { x: 8, y }, dir: East });
        }
        for y in 4..8 { // 2 to 10
            crossovers.insert(State { pos: Position { x: 12, y }, dir: East },
                              State { pos: Position { x: 0, y }, dir: East });
        }
        for x in 12..16 { // 3 to 5
            crossovers.insert(State { pos: Position { x, y: 7 }, dir: North },
                              State { pos: Position { x, y: 11 }, dir: North });
        }
        for y in 8..12 { // 4 to 7
            crossovers.insert(State { pos: Position { x: 16, y }, dir: East },
                              State { pos: Position { x: 8, y }, dir: East });
        }
        for x in 12..16 { // 5 to 3
            crossovers.insert(State { pos: Position { x, y: 12 }, dir: South },
                              State { pos: Position { x, y: 8 }, dir: South });
        }
        for x in 8..12 { // 6 to 0
            crossovers.insert(State { pos: Position { x, y: 12 }, dir: South },
                              State { pos: Position { x, y: 0 }, dir: South });
        }
        for y in 8..12 { // 7 to 4
            crossovers.insert(State { pos: Position { x: 7, y }, dir: West },
                              State { pos: Position { x: 15, y }, dir: West });
        }
        for x in 0..8 { // 8 to 12 & 9 to 11
            crossovers.insert(State { pos: Position { x, y: 8 }, dir: South },
                              State { pos: Position { x, y: 4 }, dir: South });
        }
        for y in 4..8 { // 10 to 2
            crossovers.insert(State { pos: Position { x: -1, y }, dir: West },
                              State { pos: Position { x: 11, y }, dir: West });
        }
        for x in 0..8 { // 11 to 9 & 12 to 8
            crossovers.insert(State { pos: Position { x, y: 3 }, dir: North },
                              State { pos: Position { x, y: 7 }, dir: North });
        }
        for y in 0..4 { // 13 to 1
            crossovers.insert(State { pos: Position { x: 7, y }, dir: West },
                              State { pos: Position { x: 11, y }, dir: West });
        }
    } else { // for real data
        for x in 50..100 { // 0 to 6
            crossovers.insert(State { pos: Position { x, y: -1 }, dir: North },
                              State { pos: Position { x, y: 149 }, dir: North });
        }
        for x in 100..150 { // 1 to 3
            crossovers.insert(State { pos: Position { x, y: -1 }, dir: North },
                              State { pos: Position { x, y: 49 }, dir: North });
        }
        for y in 0..50 { // 2 to 13
            crossovers.insert(State { pos: Position { x: 150, y }, dir: East },
                              State { pos: Position { x: 50, y }, dir: East });
        }
        for x in 100..150 { // 3 to 1
            crossovers.insert(State { pos: Position { x, y: 50 }, dir: South },
                              State { pos: Position { x, y: 0 }, dir: South });
        }
        for y in 50..100 { // 4 to 12
            crossovers.insert(State { pos: Position { x: 100, y }, dir: East },
                              State { pos: Position { x: 50, y }, dir: East });
        }
        for y in 100..150 { // 5 to 10
            crossovers.insert(State { pos: Position { x: 100, y }, dir: East },
                              State { pos: Position { x: 0, y }, dir: East });
        }
        for x in 50..100 { // 6 to 0
            crossovers.insert(State { pos: Position { x, y: 150 }, dir: South },
                              State { pos: Position { x, y: 0 }, dir: South });
        }
        for y in 150..200 { // 7 to 9
            crossovers.insert(State { pos: Position { x: 50, y }, dir: East },
                              State { pos: Position { x: 0, y }, dir: East });
        }
        for x in 0..50 { // 8 to 11
            crossovers.insert(State { pos: Position { x, y: 200 }, dir: South },
                              State { pos: Position { x, y: 100 }, dir: South });
        }
        for y in 150..200 { // 9 to 7
            crossovers.insert(State { pos: Position { x: -1, y }, dir: West },
                              State { pos: Position { x: 49, y }, dir: West });
        }
        for y in 100..150 { // 10 to 5
            crossovers.insert(State { pos: Position { x: -1, y }, dir: West },
                              State { pos: Position { x: 99, y }, dir: West });
        }
        for x in 0..50 { // 11 to 8
            crossovers.insert(State { pos: Position { x, y: 99 }, dir: North },
                              State { pos: Position { x, y: 199 }, dir: North });
        }
        for y in 50..100 { // 12 to 4
            crossovers.insert(State { pos: Position { x: 49, y }, dir: West },
                              State { pos: Position { x: 99, y }, dir: West });
        }
        for y in 0..50 { // 13 to 2
            crossovers.insert(State { pos: Position { x: 49, y }, dir: West },
                              State { pos: Position { x: 149, y }, dir: West });
        }
    }

    println!("{}", follow_path(board, path, &crossovers));
}

fn task2(board: &mut Vec<Vec<char>>, path: &Vec<Instruction>) {
    let mut crossovers = HashMap::<State, State>::new();

    let test_data = false; // unfortunately without automatic detection :/
    let mut outs = Vec::<State>::new();
    let mut ins = Vec::<State>::new();
    if test_data { // for test data
        { // 0 to 11
            outs.extend((8..12).map(|x| State { pos: Position { x, y: -1 }, dir: North }));
            ins.extend((0..4).rev().map(|x| State { pos: Position { x, y: 4 }, dir: South }));
        }
        { // 1 to 4
            outs.extend((0..4).map(|y| State { pos: Position { x: 12, y }, dir: East }));
            ins.extend((8..12).rev().map(|y| State { pos: Position { x: 15, y }, dir: West }));
        }
        { // 2 to 3
            outs.extend((4..8).map(|y| State { pos: Position { x: 12, y }, dir: East }));
            ins.extend((12..16).rev().map(|x| State { pos: Position { x, y: 8 }, dir: South }));
        }
        { // 3 to 2
            outs.extend((12..16).map(|x| State { pos: Position { x, y: 7 }, dir: North }));
            ins.extend((4..8).rev().map(|y| State { pos: Position { x: 11, y }, dir: West }));
        }
        { // 4 to 1
            outs.extend((8..12).map(|y| State { pos: Position { x: 16, y }, dir: East }));
            ins.extend((0..4).rev().map(|y| State { pos: Position { x: 11, y }, dir: West }));
        }
        { // 5 to 10
            outs.extend((12..16).map(|x| State { pos: Position { x, y: 12 }, dir: South }));
            ins.extend((4..8).rev().map(|y| State { pos: Position { x: 0, y }, dir: East }));
        }
        { // 6 to 9
            outs.extend((8..12).map(|x| State { pos: Position { x, y: 12 }, dir: South }));
            ins.extend((0..4).rev().map(|x| State { pos: Position { x, y: 7 }, dir: North }));
        }
        { // 7 to 8
            outs.extend((8..12).map(|y| State { pos: Position { x: 7, y }, dir: West }));
            ins.extend((4..8).rev().map(|x| State { pos: Position { x, y: 7 }, dir: North }));
        }
        { // 8 to 7
            outs.extend((4..8).map(|x| State { pos: Position { x, y: 8 }, dir: South }));
            ins.extend((8..12).rev().map(|y| State { pos: Position { x: 8, y }, dir: East }));
        }
        { // 9 to 6
            outs.extend((0..4).map(|x| State { pos: Position { x, y: 8 }, dir: South }));
            ins.extend((8..12).rev().map(|x| State { pos: Position { x, y: 11 }, dir: North }));
        }
        { // 10 to 5
            outs.extend((4..8).map(|y| State { pos: Position { x: -1, y }, dir: West }));
            ins.extend((12..16).rev().map(|x| State { pos: Position { x, y: 11 }, dir: North }));
        }
        { // 11 to 0
            outs.extend((0..4).map(|x| State { pos: Position { x, y: 3 }, dir: North }));
            ins.extend((8..12).rev().map(|x| State { pos: Position { x, y: 0 }, dir: South }));
        }
        { // 12 to 13
            outs.extend((4..8).map(|x| State { pos: Position { x, y: 3 }, dir: North }));
            ins.extend((0..4).map(|y| State { pos: Position { x: 8, y }, dir: East }));
        }
        { // 13 to 12
            outs.extend((0..4).map(|y| State { pos: Position { x: 7, y }, dir: West }));
            ins.extend((4..8).map(|x| State { pos: Position { x, y: 4 }, dir: South }));
        }
    } else { // for real data
        { // 0 to 9
            outs.extend((50..100).map(|x| State { pos: Position { x, y: -1 }, dir: North }));
            ins.extend((150..200).map(|y| State { pos: Position { x: 0, y }, dir: East }));
        }
        { // 1 to 8
            outs.extend((100..150).map(|x| State { pos: Position { x, y: -1 }, dir: North }));
            ins.extend((0..50).map(|x| State { pos: Position { x, y: 199 }, dir: North }));
        }
        { // 2 to 5
            outs.extend((0..50).map(|y| State { pos: Position { x: 150, y }, dir: East }));
            ins.extend((100..150).rev().map(|y| State { pos: Position { x: 99, y }, dir: West }));
        }
        { // 3 to 4
            outs.extend((100..150).map(|x| State { pos: Position { x, y: 50 }, dir: South }));
            ins.extend((50..100).map(|y| State { pos: Position { x: 99, y }, dir: West }));
        }
        { // 4 to 3
            outs.extend((50..100).map(|y| State { pos: Position { x: 100, y }, dir: East }));
            ins.extend((100..150).map(|x| State { pos: Position { x, y: 49 }, dir: North }));
        }
        { // 5 to 2
            outs.extend((100..150).map(|y| State { pos: Position { x: 100, y }, dir: East }));
            ins.extend((0..50).rev().map(|y| State { pos: Position { x: 149, y }, dir: West }));
        }
        { // 6 to 7
            outs.extend((50..100).map(|x| State { pos: Position { x, y: 150 }, dir: South }));
            ins.extend((150..200).map(|y| State { pos: Position { x: 49, y }, dir: West }));
        }
        { // 7 to 6
            outs.extend((150..200).map(|y| State { pos: Position { x: 50, y }, dir: East }));
            ins.extend((50..100).map(|x| State { pos: Position { x, y: 149 }, dir: North }));
        }
        { // 8 to 1
            outs.extend((0..50).map(|x| State { pos: Position { x, y: 200 }, dir: South }));
            ins.extend((100..150).map(|x| State { pos: Position { x, y: 0 }, dir: South }));
        }
        { // 9 to 0
            outs.extend((150..200).map(|y| State { pos: Position { x: -1, y }, dir: West }));
            ins.extend((50..100).map(|x| State { pos: Position { x, y: 0 }, dir: South }));
        }
        { // 10 to 13
            outs.extend((100..150).map(|y| State { pos: Position { x: -1, y }, dir: West }));
            ins.extend((0..50).rev().map(|y| State { pos: Position { x: 50, y }, dir: East }));
        }
        { // 11 to 12
            outs.extend((0..50).map(|x| State { pos: Position { x, y: 99 }, dir: North }));
            ins.extend((50..100).map(|y| State { pos: Position { x: 50, y }, dir: East }));
        }
        { // 12 to 11
            outs.extend((50..100).map(|y| State { pos: Position { x: 49, y }, dir: West }));
            ins.extend((0..50).map(|x| State { pos: Position { x, y: 100 }, dir: South }));
        }
        { // 13 to 10
            outs.extend((0..50).map(|y| State { pos: Position { x: 49, y }, dir: West }));
            ins.extend((100..150).rev().map(|y| State { pos: Position { x: 0, y }, dir: East }));
        }
    }
    outs.iter().zip(ins.iter()).for_each(|(o, i)| { crossovers.insert(*o, *i); });

    println!("{}", follow_path(board, path, &crossovers));
}

fn follow_path(board: &mut Vec<Vec<char>>, path: &Vec<Instruction>, crossovers: &HashMap<State, State>) -> i32 {
    let mut state = State {
        pos: Position {
            x: board[0].iter().position(|field| *field == '.').unwrap() as i32,
            y: 0,
        },
        dir: East,
    };

    for inst in path {
        if inst.is_rotate() {
            state.dir = state.dir.rotate(inst.rotate().unwrap());
        } else {
            let mut done = 0;
            while done < inst.step().unwrap() {
                let mut new_state = State {
                    pos: state.pos + state.dir.offset(),
                    dir: state.dir,
                };

                // use crossover?
                if crossovers.contains_key(&new_state) {
                    new_state = crossovers[&new_state];
                }

                // hitting wall? abort step
                if board[new_state.pos.y as usize][new_state.pos.x as usize] == '#' {
                    break;
                }

                state = new_state;
                done += 1;
            }
        }
    }

    return 1000 * (state.pos.y + 1) + 4 * (state.pos.x + 1) + state.dir.value();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self) -> Position {
        match self {
            North => Position { x: 0, y: -1 },
            East => Position { x: 1, y: 0 },
            South => Position { x: 0, y: 1 },
            West => Position { x: -1, y: 0 }
        }
    }

    fn rotate(&self, rotation: Rotation) -> Direction {
        match self {
            North => match rotation {
                Left => West,
                Right => East
            },
            East => match rotation {
                Left => North,
                Right => South
            },
            South => match rotation {
                Left => East,
                Right => West
            },
            West => match rotation {
                Left => South,
                Right => North
            },
        }
    }

    fn value(&self) -> i32 {
        match self {
            North => 3,
            East => 0,
            South => 1,
            West => 2
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Step(u32),
    Rotate(Rotation),
}

impl Instruction {
    fn is_step(&self) -> bool {
        match self {
            Step(_) => true,
            Rotate(_) => false
        }
    }

    fn is_rotate(&self) -> bool {
        !self.is_step()
    }

    fn step(&self) -> Option<u32> {
        match self {
            Step(val) => Some(*val),
            Rotate(_) => None
        }
    }

    fn rotate(&self) -> Option<Rotation> {
        match self {
            Step(_) => None,
            Rotate(val) => Some(*val)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}