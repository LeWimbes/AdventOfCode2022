use std::cell::{RefCell, RefMut};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

fn main() {
    let input = include_str!("day15.txt");
    let sensor_beacons: Vec<(CoordinateInterval, CoordinateInterval)> = input.lines()
        .map(|mut line| {
            let start_x1 = "Sensor at x=".len();
            let end_x1 = line.find(',').unwrap();
            let x1: i32 = line[start_x1..end_x1].parse().unwrap();
            let start_y1 = end_x1 + ", y=".len();
            let end_y1 = line.find(':').unwrap();
            let y1: i32 = line[start_y1..end_y1].parse().unwrap();

            line = &line[end_y1 + ": closest beacon is at x=".len()..line.len()];
            let part2 = line.split_once(", y=").unwrap();
            let x2: i32 = part2.0.parse().unwrap();
            let y2: i32 = part2.1.parse().unwrap();

            (CoordinateInterval { x: x1, y: y1 }, CoordinateInterval { x: x2, y: y2 })
        }).collect();

    task1(&sensor_beacons);
    task2(&sensor_beacons);
}

fn task1(sensor_beacons: &Vec<(CoordinateInterval, CoordinateInterval)>) {
    let y = 2000000;
    let intervals = RefCell::new(get_intervals(sensor_beacons, y));

    // remove coords where there are beacons (since beacons are beacons :O)
    for (_, beacon) in sensor_beacons {
        if beacon.y == y {
            remove_point(intervals.borrow_mut(), beacon.x);
        }
    }

    let result = intervals.borrow().iter()
        .fold(0, |acc, int| acc + int.len());
    println!("{}", result);
}

fn task2(sensor_beacons: &Vec<(CoordinateInterval, CoordinateInterval)>) {
    let max: usize = 4000000;
    let range = CoordinateInterval { x: 0, y: max as i32 };

    let mut beacon = CoordinateInterval { x: 0, y: 0 };
    for y in 0..=max {
        let mut intervals = get_intervals(sensor_beacons, y as i32);
        intervals.retain(|int| int.intersects(&range));

        if intervals.len() == 1 {
            if intervals[0].x > 0 {
                beacon = CoordinateInterval { x: 0, y: y as i32 };
                break;
            } else if intervals[0].y < max as i32 {
                beacon = CoordinateInterval { x: max as i32, y: y as i32 };
                break;
            }
        } else {
            beacon = CoordinateInterval { x: intervals[0].y + 1, y: y as i32 };
            break;
        }
    }

    println!("{}", beacon.x as u64 * max as u64 + beacon.y as u64);
}

fn remove_point(mut intervals: RefMut<Vec<CoordinateInterval>>, point: i32) {
    let mut interval: Option<CoordinateInterval> = None;
    intervals.retain(|int| {
        if int.contains(point) {
            interval = Some(int.clone());
            false
        } else {
            true
        }
    });

    if interval.is_none() {
        return;
    }
    let int = interval.unwrap();

    if int.x == point && int.y == point {
        return;
    }
    if int.x == point {
        let int2 = CoordinateInterval { x: int.x + 1, y: int.y };
        let pos = intervals.binary_search(&int2).err().unwrap();
        intervals.insert(pos, int2);
    } else if int.y == point {
        let int1 = CoordinateInterval { x: int.x, y: int.y - 1 };
        let pos = intervals.binary_search(&int1).err().unwrap();
        intervals.insert(pos, int1);
    } else {
        let int1 = CoordinateInterval { x: int.x, y: point - 1 };
        let pos = intervals.binary_search(&int1).err().unwrap();
        intervals.insert(pos, int1);

        let int2 = CoordinateInterval { x: point + 1, y: int.y };
        let pos = intervals.binary_search(&int2).err().unwrap();
        intervals.insert(pos, int2);
    }
}

fn insert_interval(intervals: &mut Vec<CoordinateInterval>, interval: &CoordinateInterval) {
    let mut intersecting: Vec<CoordinateInterval> = Vec::new();
    intervals.retain(|int| {
        if int.intersects(interval) {
            intersecting.push(int.clone());
            false
        } else {
            true
        }
    });

    let mut result = interval.clone();
    for int in intersecting {
        result = result.combine(&int);
    }
    let pos = intervals.binary_search(&result).err().unwrap();
    intervals.insert(pos, result);
}

fn get_intervals(sensor_beacons: &Vec<(CoordinateInterval, CoordinateInterval)>, y: i32) -> Vec<CoordinateInterval> {
    let mut intervals: Vec<CoordinateInterval> = Vec::new();
    for (sensor, beacon) in sensor_beacons {
        let dist = sensor.manh_dist(beacon);
        let offset = sensor.y.abs_diff(y);
        if offset > dist { // sensor has no influence
            continue;
        }

        let radius = (dist - offset) as i32;
        let int = CoordinateInterval { x: sensor.x - radius, y: sensor.x + radius };
        insert_interval(&mut intervals, &int);
    }
    return intervals;
}

#[derive(Clone, Copy)]
struct CoordinateInterval {
    x: i32,
    y: i32,
}

impl CoordinateInterval {
    fn manh_dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn len(&self) -> usize {
        (self.y - self.x + 1) as usize
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x <= other.y && self.y >= other.x
    }

    fn contains(&self, other: i32) -> bool {
        other >= self.x && other <= self.y
    }

    fn combine(&self, other: &Self) -> Self {
        CoordinateInterval {
            x: self.x.min(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl Eq for CoordinateInterval {}

impl PartialEq<Self> for CoordinateInterval {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd<Self> for CoordinateInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.x < other.x {
            Ordering::Less
        } else if self.x > other.x {
            Ordering::Greater
        } else if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl Ord for CoordinateInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for CoordinateInterval {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Debug for CoordinateInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}