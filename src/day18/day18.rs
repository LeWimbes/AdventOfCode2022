use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};
use std::rc::Rc;

fn main() {
    let input = include_str!("day18.txt");
    let cubes: HashSet<Cube> = input.lines().map(|line| {
        let mut parts = line.splitn(3, ',')
            .map(|entry| entry.parse::<i32>().unwrap());
        build_cube(parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap())
    }).collect();
    let cubes = Rc::new(RefCell::new(cubes));

    task1(cubes.borrow());
    task2(cubes.borrow());
}

fn task1(cubes: &RefCell<HashSet<Cube>>) {
    let mut air: usize = 0;
    for cube in cubes.borrow().iter() {
        air += air_around_cube(&cube, &cubes.borrow()).len();
    }
    println!("{}", air);
}

fn task2(cubes: &RefCell<HashSet<Cube>>) {
    // find bounding box
    let mut min: Cube = build_cube(i32::MAX, i32::MAX, i32::MAX);
    let mut max: Cube = build_cube(i32::MIN, i32::MIN, i32::MIN);
    for cube in cubes.borrow().iter() {
        min = min.min(cube);
        max = max.max(cube);
    }
    min = min - 1;
    max = max + 1;

    let mut exterior: usize = 0;
    let mut air_map = HashMap::<Cube, bool>::new(); // is air exterior?

    for cube in cubes.borrow().iter() {
        let air = air_around_cube(&cube, &cubes.borrow());
        for c in air {
            if !air_map.contains_key(&c) {
                air_map.insert(c, is_exterior(c, &cubes.borrow(), &min, &max));
            }

            if air_map[&c] { exterior += 1 }
        }
    }

    println!("{}", exterior);
}

fn is_exterior(air: Cube, cubes: &HashSet<Cube>, min: &Cube, max: &Cube) -> bool {
    let mut visited = HashSet::<Cube>::new();
    let mut queue = VecDeque::<Cube>::new();
    visited.insert(air);
    queue.push_back(air);

    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();

        if !cube.in_box(min, max) {
            return true;
        }

        for c in cube.neighbors() {
            if visited.contains(&c) { continue; }

            if !cubes.contains(&c) {
                visited.insert(c);
                queue.push_back(c);
            }
        }
    }
    return false;
}

fn air_around_cube(cube: &Cube, cubes: &HashSet<Cube>) -> Vec<Cube> {
    let mut air = cube.neighbors();
    air.retain(|c| !cubes.contains(c));
    air
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn min(&self, other: &Self) -> Self {
        Cube {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Cube {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    fn neighbors(&self) -> Vec<Cube> {
        let left = build_cube(self.x - 1, self.y, self.z);
        let right = build_cube(self.x + 1, self.y, self.z);
        let back = build_cube(self.x, self.y - 1, self.z);
        let front = build_cube(self.x, self.y + 1, self.z);
        let down = build_cube(self.x, self.y, self.z - 1);
        let up = build_cube(self.x, self.y, self.z + 1);

        vec!(left, right, back, front, down, up)
    }

    fn in_box(&self, min: &Self, max: &Self) -> bool {
        !(self.x <= min.x || self.x >= max.x ||
            self.y <= min.y || self.y >= max.y ||
            self.z <= min.z || self.z >= max.z)
    }
}

impl Add<i32> for Cube {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Cube {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub<i32> for Cube {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Cube {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

fn build_cube(x: i32, y: i32, z: i32) -> Cube {
    Cube { x, y, z }
}