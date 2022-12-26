fn main() {
    let input = include_str!("day20.txt");
    let coordinates: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    task1(&coordinates);
    task2(&coordinates);
}

fn task1(coordinates: &Vec<i64>) {
    let coord = calc_grove_coordinate(coordinates, 1, 1);
    println!("{}", coord.0 + coord.1 + coord.2);
}

fn task2(coordinates: &Vec<i64>) {
    let coord = calc_grove_coordinate(coordinates, 811589153, 10);
    println!("{}", coord.0 + coord.1 + coord.2);
}

fn calc_grove_coordinate(coordinates: &Vec<i64>, key: i64, iterations: usize) -> (i64, i64, i64) {
    let coordinates = coordinates.iter().map(|coord| coord * key).enumerate().collect::<Vec<_>>();
    let len = coordinates.len() as i64;
    let mut mixed = coordinates.iter().collect::<Vec<_>>();
    for _ in 0..iterations {
        for coord in coordinates.iter() {
            let i = mixed.iter().position(|it| it == &coord).unwrap();
            mixed.remove(i);
            let new_pos = (i as i64 + coord.1).rem_euclid(len - 1);
            if new_pos == 0 {
                mixed.insert(mixed.len(), coord);
            } else {
                mixed.insert(new_pos as usize, coord);
            }
        }
    }

    let zero = mixed.iter().position(|it| it.1 == 0).unwrap();

    (mixed[(zero + 1000) % mixed.len()].1, mixed[(zero + 2000) % mixed.len()].1, mixed[(zero + 3000) % mixed.len()].1)
}