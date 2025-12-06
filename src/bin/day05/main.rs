use std::{collections::HashSet, env::args, io::Read, ops::RangeInclusive};

#[derive(Debug)]
struct PuzzleInput {
    ranges: Vec<RangeInclusive<i64>>,
    ingredients: Vec<i64>,
}

impl PuzzleInput {
    fn parse<'a>(mut lines: impl Iterator<Item=&'a str>) -> Self {
        let mut result = PuzzleInput{
            ranges: Vec::new(),
            ingredients: Vec::new(),
        };
        loop {
            let line = lines.next();
            let line = match line {
                None => panic!("no ingredients??"),
                Some("") => break,
                Some(line) => line,
            };
            let (lhs, rhs) = line.split_once("-").unwrap();
            let lhs: i64 = lhs.parse().unwrap();
            let rhs: i64 = rhs.parse().unwrap();
            let range = lhs..=rhs;
            result.ranges.push(range);
        }
        loop {
            let line = lines.next();
            let line = match line {
                None => break,
                Some(line) => line,
            };
            let ingredient: i64 = line.parse().unwrap();
            result.ingredients.push(ingredient);
        }
        result
    }
}

fn part1() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();

    let puzzle_input = PuzzleInput::parse(lines.iter().map(|s| s.as_str()));
    println!("{:?}", &puzzle_input);

    let mut result = 0;

    for ingredient in puzzle_input.ingredients {
        for range in &puzzle_input.ranges {
            if range.contains(&ingredient) {
                result += 1;
                break;
            }
        }
    }

    println!("{}", result);
}

fn part2() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();

    let mut puzzle_input = PuzzleInput::parse(lines.iter().map(|s| s.as_str()));
    puzzle_input.ranges.sort_by(|a, b| {
        if a.start() != b.start() {
            return a.start().cmp(b.start());
        } else {
            return a.end().cmp(b.end());
        }
    });

    for range in &puzzle_input.ranges {
        println!("{:?}", range);
    }

    println!();

    let mut result = 0;

    for i in 0..puzzle_input.ranges.len() {
        let start = *puzzle_input.ranges[i].start();
        let mut end = *puzzle_input.ranges[i].end();

        let is_first = i == 0;
        let is_last = i == puzzle_input.ranges.len() - 1;
        
        if !is_first {
            end = i64::max(*puzzle_input.ranges[i-1].end(), end);   
        }
        if !is_last {
            end = i64::min(end, *puzzle_input.ranges[i+1].start()-1);
        }

        println!("{} -> {}", start, end);

        result += end - start + 1;
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
