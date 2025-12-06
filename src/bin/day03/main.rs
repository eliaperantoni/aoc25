use std::env::args;

fn part1() {
    let stdin = std::io::stdin();
    let mut result = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let numbers = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        let (left_digit_idx, left_digit) = numbers[0..numbers.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| {
                if a.1 != b.1 {
                    a.1.cmp(&b.1)
                } else {
                    b.0.cmp(&a.0)
                }
            })
            .unwrap();
        let right_digit = numbers[left_digit_idx + 1..]
            .iter()
            .copied()
            .max()
            .unwrap();
        let joltage = left_digit * 10 + right_digit;
        result += joltage;
    }
    println!("{}", result);
}

fn part2() {
    let stdin = std::io::stdin();
    let mut result: u64 = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let numbers = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();

        let mut left_boundary = 0;
        let mut right_boundary = numbers.len() - 11;

        let mut joltage: u64 = 0;

        for i in 0..12 {
            let (digit_idx, digit) = numbers[left_boundary..right_boundary]
                .iter()
                .copied()
                .enumerate()
                .max_by(|a, b| {
                    if a.1 != b.1 {
                        a.1.cmp(&b.1)
                    } else {
                        b.0.cmp(&a.0)
                    }
                })
                .unwrap();

            left_boundary += digit_idx + 1;
            right_boundary += 1;

            joltage = joltage + digit * 10_u64.pow(11 - i as u32);
        }

        println!("Joltage: {}", joltage);

        result += joltage;
    }
    println!("{}", result);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
