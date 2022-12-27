fn main() {
    let input = include_str!("day25.txt");
    let sum: u64 = input.lines().map(|snafu| snafu_to_decimal(&snafu.to_string())).sum();
    let snafu_sum = decimal_to_snafu(sum);
    println!("{}", snafu_sum);
}

fn snafu_to_decimal(snafu: &String) -> u64 {
    let len = snafu.len() - 1;
    let mut decimal: i64 = 0;
    for (i, c) in snafu.chars().enumerate() {
        decimal += (5 as i64).pow((len - i) as u32) * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => 0
        }
    }
    return decimal as u64;
}

fn decimal_to_snafu(decimal: u64) -> String {
    let mut decimal = decimal;
    let mut snafu_vec = Vec::<i8>::new();
    while decimal != 0 {
        let rem = decimal % 5;
        snafu_vec.push(rem as i8);
        decimal /= 5;
    }

    let mut index: usize = 0;
    while index < snafu_vec.len() {
        if snafu_vec[index] > 2 {
            if snafu_vec[index] == 3 {
                snafu_vec[index] = -2;
            } else if snafu_vec[index] == 4 {
                snafu_vec[index] = -1;
            } else if snafu_vec[index] == 5 {
                snafu_vec[index] = 0;
            }

            if index == snafu_vec.len() - 1 {
                snafu_vec.push(1);
            } else {
                snafu_vec[index + 1] += 1;
            }
        }
        index += 1;
    }

    let mut snafu = String::new();
    for i in (0..index).rev() {
        snafu.push(match snafu_vec[i] {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("Unexpected value {}!", snafu_vec[i])
        });
    }
    return snafu;
}