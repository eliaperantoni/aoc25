use std::{env::args, io::Read};

fn part1() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();

    let m = lines[0].len();
    let mut rays: Vec<_> = (0..m).map(|_| false).collect();
    let S_idx = lines[0].chars().enumerate().find_map(|(idx, c)| {
        if c == 'S' {
            Some(idx)
        } else {
            None
        }
    }).unwrap();
    rays[S_idx] = true;

    let mut result = 0;

    for (line_idx, line) in lines[1..].iter().enumerate() {
        let mut new_rays = rays.clone();
        for i in 0..m {
            if line.as_bytes()[i] == b'^' && rays[i] {
                new_rays[i] = false;
                if i > 0 {
                    new_rays[i-1] = true;
                }
                if i < m-1 {
                    new_rays[i+1] = true;
                }
                result += 1;
            }
        }
        rays = new_rays;
    }

    println!("{}", result);
}

fn part2() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();

    let m = lines[0].len();
    let mut rays: Vec<_> = (0..m).map(|_| 0usize).collect();
    let S_idx = lines[0].chars().enumerate().find_map(|(idx, c)| {
        if c == 'S' {
            Some(idx)
        } else {
            None
        }
    }).unwrap();
    rays[S_idx] = 1;

    for line in &lines[1..] {
        let mut new_rays = rays.clone();
        for i in 0..m {
            if line.as_bytes()[i] == b'^' {
                new_rays[i] = 0;
                if i > 0 {
                    new_rays[i-1] += rays[i];
                }
                if i < m-1 {
                    new_rays[i+1] += rays[i];
                }
            }
        }
        rays = new_rays;
    }

    let result: usize = rays.iter().sum();
    println!("{}", result);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
