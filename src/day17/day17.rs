use std::collections::HashMap;

fn main() {
    let input = include_str!("day17.txt");

    let jets: Vec<char> = input.lines().next().unwrap().chars().collect();

    // basically rotating everything by 90° clockwise to easily extend the chamber
    let rocks: Vec<Vec<Vec<bool>>> = vec![vec![vec![true],
                                               vec![true],
                                               vec![true],
                                               vec![true]],
                                          vec![vec![false, true, false],
                                               vec![true, true, true],
                                               vec![false, true, false]],
                                          vec![vec![true, false, false],
                                               vec![true, false, false],
                                               vec![true, true, true]],
                                          vec![vec![true, true, true, true]],
                                          vec![vec![true, true],
                                               vec![true, true]]];


    task1(&jets, &rocks);
    task2(&jets, &rocks);
}

fn task1(jets: &Vec<char>, rocks: &Vec<Vec<Vec<bool>>>) {
    println!("{}", simulate_height(jets, rocks, 2022))
}

fn task2(jets: &Vec<char>, rocks: &Vec<Vec<Vec<bool>>>) {
    println!("{}", simulate_height(jets, rocks, 1000000000000))
}

fn simulate_height(jets: &Vec<char>, rocks: &Vec<Vec<Vec<bool>>>, rock_count: usize) -> u64 {
    let mut chamber: Vec<Vec<bool>> = vec![vec![false; 3]; 7];
    let cols_to_save: usize = 32; // might be to low for some jet patterns
    let mut states = HashMap::<(usize, usize, Vec<bool>), (u64, usize)>::new();
    let mut skipped = false;
    let mut bonus_height: u64 = 0;

    let mut height: u64 = 0;
    let mut j = 0;
    let mut i: usize = 0;
    while i < rock_count {
        if !skipped && i >= cols_to_save { // look for period
            let next_rock = i % rocks.len();
            let next_jet = j % jets.len();

            let mut chamber_state = Vec::<bool>::new();
            for y in 0..chamber.len() {
                chamber_state.extend_from_slice(
                    &chamber[y][(height as usize - cols_to_save)..height as usize]);
            }

            let state = (next_rock, next_jet, chamber_state);
            if states.contains_key(&state) {
                let (old_height, old_i) = states.get(&state).unwrap();

                let skipped_rocks = i - old_i;
                let skipped_height = height - old_height;

                let steps = (rock_count - i) / skipped_rocks;
                i += skipped_rocks * steps;
                bonus_height = skipped_height * steps as u64;

                skipped = true;
            } else {
                states.insert(state, (height, i));
            }
        }

        let rock = &rocks[i % rocks.len()];

        let mut pos = spawn_rock(&mut chamber, rock, height);

        let mut blocked = false;
        while !blocked {
            let tmp = next_pos(&chamber, rock, pos, jets[j % jets.len()]);
            match tmp {
                Ok(val) => pos = val,
                Err(val) => {
                    pos = val;
                    blocked = true;
                }
            }
            j += 1;
        }

        imprint_rock(&mut chamber, rock, pos);
        height = height.max((pos.0 + rock[0].len()) as u64);
        i += 1;
    }
    return height + bonus_height;
}

fn spawn_rock(chamber: &mut Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, height: u64) -> (usize, usize) {
    let x = (height + 3) as usize;
    let y = 2 as usize;

    for y in 0..chamber.len() {
        chamber[y].resize(x + rock[0].len(), false);
    }

    return (x, y);
}

fn next_pos(chamber: &Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize), jet: char) -> Result<(usize, usize), (usize, usize)> {
    let mut next = pos;

    // left/right or in this case up/down
    if jet == '<' && next.1 != 0 {
        next.1 -= 1;
        if intersects(chamber, rock, next) {
            next.1 += 1;
        }
    } else if jet == '>' && next.1 + rock.len() != chamber.len() {
        next.1 += 1;
        if intersects(chamber, rock, next) {
            next.1 -= 1;
        }
    }

    // down or in this case left
    let mut blocked = next.0 == 0;
    if !blocked {
        next.0 -= 1;
        if intersects(chamber, rock, next) {
            next.0 += 1;
            blocked = true;
        }
    }

    return if blocked { Err(next) } else { Ok(next) };
}

fn intersects(chamber: &Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    for y in 0..rock.len() {
        for x in 0..rock[0].len() {
            if rock[y][x] && chamber[pos.1 + y][pos.0 + x] {
                return true;
            }
        }
    }
    return false;
}

fn imprint_rock(chamber: &mut Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) {
    for y in 0..rock.len() {
        for x in 0..rock[0].len() {
            if rock[y][x] {
                chamber[pos.1 + y][pos.0 + x] = true;
            }
        }
    }
}