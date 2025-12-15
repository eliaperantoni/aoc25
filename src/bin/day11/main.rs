use std::{collections::{HashMap, HashSet}, env::args, io::stdin};

#[derive(Debug, Clone, Default)]
struct Input {
    edges: Vec<(String, String)>,
}

fn parse_input() -> Input {
    let mut input = Input::default();
    for line in stdin().lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once(":").unwrap();
        for to in to.trim().split_ascii_whitespace() {
            input.edges.push((from.to_string(), to.to_string()))
        }
    }
    input
}

fn ways_of_reaching_out(currently_at: &str, input: &Input) -> i64 {
    if currently_at == "out" {
        return 1;
    }
    let descendants = input
        .edges
        .iter()
        .filter(|(from, _)| from == &currently_at)
        .map(|(_, to)| to);
    let mut partial_result = 0;

    for descendant in descendants {
        partial_result += ways_of_reaching_out(descendant, input);
    }

    partial_result
}

#[derive(Debug, Default, Clone)]
struct ReachResult {
    through_neither: i64,
    through_fft: i64,
    through_dac: i64,
    through_both: i64,
}

impl std::ops::Add for ReachResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            through_neither: self.through_neither + rhs.through_neither,
            through_fft: self.through_fft + rhs.through_fft,
            through_dac: self.through_dac + rhs.through_dac,
            through_both: self.through_both + rhs.through_both,
        }
    }
}

type MEMO = HashMap<String, ReachResult>;

fn ways_of_reaching(currently_at: &str, input: &Input, memo: &mut MEMO) -> ReachResult {
    if currently_at == "out" {
        return ReachResult {
            through_neither: 1,
            ..Default::default()
        };
    }

    if let Some(reach_result) = memo.get(currently_at) {
        return reach_result.clone();
    }

    let descendants = input
        .edges
        .iter()
        .filter(|(from, _)| from == &currently_at)
        .map(|(_, to)| to);

    let mut reach_result = ReachResult::default();

    for descendant in descendants {
        let partial_result = ways_of_reaching(descendant, input, memo);
        if currently_at == "fft" {
            assert_eq!(partial_result.through_fft, 0);
            assert_eq!(partial_result.through_both, 0);
            reach_result = ReachResult {
                through_neither: 0,
                through_fft: reach_result.through_fft + partial_result.through_neither,
                through_dac: 0,
                through_both: reach_result.through_both + partial_result.through_dac,
            };
        } else if currently_at == "dac" {
            assert_eq!(partial_result.through_dac, 0);
            assert_eq!(partial_result.through_both, 0);
            reach_result = ReachResult {
                through_neither: 0,
                through_fft: 0,
                through_dac: reach_result.through_dac + partial_result.through_neither,
                through_both: reach_result.through_both + partial_result.through_fft,
            };
        } else {
            reach_result = ReachResult {
                through_neither: reach_result.through_neither + partial_result.through_neither,
                through_fft: reach_result.through_fft + partial_result.through_fft,
                through_dac: reach_result.through_dac + partial_result.through_dac,
                through_both: reach_result.through_both + partial_result.through_both,
            };
        }
    }

    memo.insert(currently_at.to_string(), reach_result.clone());

    reach_result
}

fn part1() {
    let input = parse_input();
    for (from, to) in &input.edges {
        println!("{} -> {}", from, to);
    }
    println!();

    let paths = ways_of_reaching_out("you", &input);
    println!("paths: {}", paths);
}

fn part2() {
    let input = parse_input();
    for (from, to) in &input.edges {
        println!("{} -> {}", from, to);
    }
    println!();

    let mut memo = MEMO::new();
    let paths = dbg!(ways_of_reaching("svr", &input, &mut memo)).through_both;
    println!("paths: {}", paths);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
