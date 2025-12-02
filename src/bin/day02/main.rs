use std::{env::args, io::Read};

fn is_invalid_id_part1(id: i64) -> bool {
    let id = id.to_string();
    if id.len() % 2 != 0 {
        return false;
    }
    let half = id.len() / 2;
    let (first_half, second_half) = id.split_at(half);
    first_half == second_half
}

fn is_invalid_id_part2(id: i64) -> bool {
    let id = id.to_string();
    for pref_len in 1..=(id.len() / 2) {
        let pat = &id[0..pref_len];
        let mut matches = true;
        let mut idx: usize = 0;
        while idx < id.len() {
            if id.as_bytes()[idx] != pat.as_bytes()[idx % pref_len] {
                matches = false;
                break;
            }
            idx += 1;
        }
        if matches && idx == id.len() && id.len() >= 2 * pat.len() && id.len() % pat.len() == 0 {
            return true;
        }
    }
    return false;
}

fn solve(is_invalid_id: fn(i64) -> bool) {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    input.retain(|c| !c.is_whitespace());

    let ranges = input.split(",").map(|range_str| {
        let (from, to) = range_str.split_once("-").unwrap();
        let from: i64 = from.parse().unwrap();
        let to: i64 = to.parse().unwrap();
        from..=to
    }).collect::<Vec<_>>();

    let mut result: i64 = 0;

    for range in ranges {
        for num in range {
            if is_invalid_id(num) {
                result += num as i64;
            }
        }
    }

    println!("{}", result);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return solve(is_invalid_id_part1),
        Some("2") => return solve(is_invalid_id_part2),
        _ => {}
    }
}