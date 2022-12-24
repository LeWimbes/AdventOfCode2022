use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use regex::Regex;

fn main() {
    let input = include_str!("day16.txt");
    let rx_num = Regex::new(r"\d+").unwrap();
    let rx_label = Regex::new(r"[A-Z][A-Z]").unwrap();

    let mut valve_index = HashMap::<String, usize>::new();
    let mut index_valve = HashMap::<usize, String>::new();
    let mut valves: HashMap<usize, Valve> = input.lines().enumerate().map(|(i, line)| {
        let flow_rate: u32 = rx_num.captures(line).unwrap()[0].parse().unwrap();

        let mut caps = rx_label.captures_iter(line);
        let label = caps.next().unwrap()[0].to_string();
        let neighbors: ValveWrapper = RefCell::new(HashSet::new());
        for cap in caps {
            neighbors.borrow_mut().insert(cap[0].to_string());
        }

        let valve = Valve { label: label.clone(), flow_rate, neighbors };
        valve_index.insert(label.clone(), i);
        index_valve.insert(i, label.clone());
        (i, valve)
    }).collect();

    // Floyd Warshall
    let mut distance = vec![vec![10000 as u32; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        for j in 0..valves.len() {
            if i == j {
                distance[i][j] = 0;
            } else if valves[&i].neighbors.borrow().contains(&index_valve[&j]) {
                distance[i][j] = 1;
            }
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
            }
        }
    }

    // remove unnecessary valves
    for i in (0..valves.len()).rev() {
        let label = &index_valve[&i].clone();
        if valves[&i].flow_rate == 0 && label != "AA" {
            for j in i + 1..valves.len() {
                let label = &index_valve[&j].clone();
                valve_index.insert(label.clone(), j - 1);
                index_valve.remove(&j);
                index_valve.insert(j - 1, label.clone());
                let valve = valves.remove(&j).unwrap();
                valves.insert(j - 1, valve);
            }
            distance.remove(i);
            for j in 0..distance.len() {
                distance[j].remove(i);
            }
        }
    }

    let start = valve_index["AA"];
    task1(&distance, &valves, start);
    task2(&distance, &valves, start);
}

fn task1(distance: &Vec<Vec<u32>>, valves: &HashMap<usize, Valve>, start: usize) {
    println!("{}", dfs(start, 0, 0, &mut HashSet::<usize>::new(), distance, valves, 30, true, start));
}

fn task2(distance: &Vec<Vec<u32>>, valves: &HashMap<usize, Valve>, start: usize) {
    println!("{}", dfs(start, 0, 0, &mut HashSet::<usize>::new(), distance, valves, 26, false, start));
}

fn dfs(cur: usize, time: u32, flow: u32, opened: &mut HashSet<usize>, distance: &Vec<Vec<u32>>, valves: &HashMap<usize, Valve>, max_time: u32, elephant_used: bool, aa: usize) -> u32 {
    let mut max = flow + get_flow(opened, valves) * (max_time - time);

    if !elephant_used {
        let mut elephant_valves = valves.clone();
        for open in opened.iter() {
            elephant_valves.remove(open);
        }

        let mut elephant_opened = HashSet::<usize>::new();
        let max_elephant = dfs(aa, 0, 0, &mut elephant_opened, distance, &elephant_valves, max_time, true, aa);
        max = flow + get_flow(opened, valves) * (max_time - time) + max_elephant;
    }

    for next in valves.keys() {
        if opened.contains(next) { continue; } // already opened

        let time_delta = distance[cur][*next] + 1;

        if time + time_delta >= max_time { continue; } // to much time spend

        let new_flow = flow + time_delta * get_flow(opened, valves);
        opened.insert(*next);

        let value = dfs(*next, time + time_delta, new_flow, opened, distance, valves, max_time, elephant_used, aa);
        if max < value { max = value; }
        opened.remove(next);
    }
    return max;
}

fn get_flow(opened: &HashSet<usize>, valves: &HashMap<usize, Valve>) -> u32 {
    opened.iter().fold(0, |acc, valve|
        acc + valves[valve].flow_rate,
    )
}

type ValveWrapper = RefCell<HashSet<String>>;

#[derive(Clone)]
struct Valve {
    label: String,
    flow_rate: u32,
    neighbors: ValveWrapper,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, ", self.label, self.flow_rate).expect("");
        let mut dl = f.debug_list();
        for n in self.neighbors.borrow().iter() {
            dl.entry(&n);
        }
        dl.finish().expect("");
        write!(f, ")")
    }
}