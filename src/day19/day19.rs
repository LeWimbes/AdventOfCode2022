use regex::Regex;

fn main() {
    let input = include_str!("day19.txt");
    let rx: Regex = Regex::new(r"\d+").unwrap();
    let blueprints: Vec<Blueprint> = input.lines().map(|line| {
        let ints: Vec<u64> = rx.captures_iter(line).map(|int| int[0].parse().unwrap()).collect();

        Blueprint {
            recipes: [
                [ints[1], 0, 0, 0],
                [ints[2], 0, 0, 0],
                [ints[3], ints[4], 0, 0],
                [ints[5], 0, ints[6], 0]
            ]
        }
    }).collect();

    task1(&blueprints);
    task2(&blueprints);
}

fn task1(blueprints: &Vec<Blueprint>) {
    let geodes = blueprints.iter().map(|blueprint| max_geodes(blueprint, 24));
    let qualities = geodes.enumerate().map(|(i, geodes)| (i + 1) as u64 * geodes);
    let sum = qualities.sum::<u64>();
    println!("{}", sum);
}

fn task2(blueprints: &Vec<Blueprint>) {
    let geodes = blueprints.iter().take(3).map(|blueprint| max_geodes(blueprint, 32));
    let product = geodes.product::<u64>();
    println!("{}", product);
}

fn max_geodes(blueprint: &Blueprint, time_limit: u64) -> u64 {
    // only one robot can be build per turn
    // so we only need enough robots to produce any other robot
    let mut max_robots_needed = [0; 4];
    for i in 0..=2 {
        max_robots_needed[i] = blueprint.recipes.iter().map(|recipe| recipe[i]).max().unwrap();
    }
    max_robots_needed[3] = u64::MAX; // we want as many geode robots as possible!

    let mut best: u64 = 0;
    geodes_dfs(initial_state(), blueprint, &mut best, &max_robots_needed, time_limit);
    return best;
}

fn geodes_dfs(state: State, blueprint: &Blueprint, best: &mut u64, max_robots_needed: &[u64; 4], time_limit: u64) {
    for i in 0..blueprint.recipes.len() {
        // already got enough robots of this type to produce any robot every turn?
        if state.robots[i] == max_robots_needed[i] {
            continue;
        }

        let recipe = blueprint.recipes[i];

        // skip some minutes until there are enough resources to build the robot
        let wait_time = (0..state.resources.len()).map(|res| {
            if recipe[res] <= state.resources[res] {
                0
            } else if state.robots[res] == 0 {
                time_limit + 1 // no robot yet -> can't determine wait time
            } else {
                (recipe[res] - state.resources[res] + state.robots[res] - 1) / state.robots[res]
            }
        }).max().unwrap();
        let new_time = state.time + wait_time + 1;
        if new_time >= time_limit { continue; }

        let mut new_res = [0; 4];
        for j in 0..4 {
            new_res[j] = state.resources[j] + state.robots[j] * (wait_time + 1) - recipe[j];
        }
        let mut new_robots = state.robots;
        new_robots[i] += 1;

        let time_left = time_limit - new_time;
        let will_produce = new_robots[3] * time_left;
        // if every step left produces a new geode robot
        // (time_left - 1) + (time_left - 2) + ... + 1
        let might_produce = (time_left * (time_left - 1)) / 2;
        if *best > new_res[3] + will_produce + might_produce {
            continue;
        }

        let new_state = State { time: new_time, resources: new_res, robots: new_robots };
        geodes_dfs(new_state, blueprint, best, max_robots_needed, time_limit);
    }

    // update best
    let time_left = time_limit - state.time;
    let will_produce = state.robots[3] * time_left;
    *best = (*best).max(state.resources[3] + will_produce);
}

#[derive(Debug)]
struct Blueprint {
    recipes: [[u64; 4]; 4], // Robots: Ore, Clay, Obsidian, Geode; Resources: Ore, Clay, Obsidian, Geode
}

#[derive(Debug)]
struct State {
    time: u64,
    resources: [u64; 4],
    robots: [u64; 4],
}

fn initial_state() -> State {
    State {
        time: 0,
        resources: [0; 4],
        robots: [1, 0, 0, 0],
    }
}