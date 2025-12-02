use std::env::args;

fn part1() {
    let stdin = std::io::stdin();
    let mut zeros = 0;
    let mut dial = 50;
    for line in stdin.lines() {
        let line = line.unwrap();
        let (dir, amount) = line.split_at(1);
        let mut amount: i32 = amount.parse().unwrap();
        if dir == "L" {
            amount = -amount;
        }
        dial = (dial + amount).rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
    }
    println!("{}", zeros);
}

fn part2() {
    let stdin = std::io::stdin();
    let mut zeros = 0;
    let mut dial = 50;
    for line in stdin.lines() {
        let line = line.unwrap();
        let (dir, amount) = line.split_at(1);
        let mut amount: i32 = amount.parse().unwrap();
        if dir == "L" {
            amount = -amount;
        }
        zeros += amount.abs() / 100;
        amount = amount % 100;
        if dial != 0 && (dial + amount <= 0 || dial + amount >= 100) {
            zeros += 1;
        }
        dial = (dial + amount).rem_euclid(100);
    }
    println!("{}", zeros);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}