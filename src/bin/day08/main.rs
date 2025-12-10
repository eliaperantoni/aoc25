use std::{cmp::Reverse, collections::HashMap, env::args, fmt::Display, num::{ParseFloatError, ParseIntError}, str::FromStr};

use itertools::Itertools;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

type Float = OrderedFloat<f64>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: Float,
    y: Float,
    z: Float,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split(",").collect();
        if pieces.len() != 3 {
            return Err("need 3 pieces".to_string());
        }
        Ok(Point {
            x: pieces[0]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?,
            y: pieces[1]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?,
            z: pieces[2]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?,
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl Point {
    fn dist(&self, other: &Self) -> Float {
        Float::from(((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt())
    }
}

fn parse_input() -> Vec<Point> {
    let mut points = Vec::new();

    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let line = line.unwrap();
        let point = Point::from_str(&line).unwrap();
        points.push(point);
    }

    points
}

fn part1() {
    let points = parse_input();
    for point in &points {
        println!("{}", point);
    }

    let mut pq = PriorityQueue::<_, _>::new();
    for a in 0..(points.len()-1) {
        for b in (a+1)..points.len() {
            let dist = points[a].dist(&points[b]);
            pq.push((points[a], points[b]), Reverse(dist));
        }
    }

    let mut circuits: HashMap<Point, i32> = HashMap::new();

    let mut i = 0;
    while i < 1000 {
        let ((a, b), _) = pq.pop().unwrap();
        if let Some(&prev_i) = circuits.get(&a) {
            for (_, j) in &mut circuits {
                if *j == prev_i {
                    *j = i;
                }
            }
        }
        if let Some(&prev_i) = circuits.get(&b) {
            for (_, j) in &mut circuits {
                if *j == prev_i {
                    *j = i;
                }
            }
        }
        circuits.insert(a, i);
        circuits.insert(b, i);
        println!("Connect {} to {}", &a, &b);
        debug_print_circuits(&circuits);
        i +=1;
    }

    let counts = circuits.values().copied().counts();
    let mut top_counts = counts.into_values().collect_vec();
    top_counts.sort();
    top_counts.reverse();
    let result = top_counts.into_iter().take(3).reduce(|acc, e| acc * e).unwrap();
    println!("{}", result);
}

fn debug_print_circuits(circuits: &HashMap<Point, i32>) {
    let counts = circuits.values().copied().counts();
    for (_, size) in counts {
        println!("Circuit with {} boxes", size);
    }
}

fn is_one_circuit(circuits: &HashMap<Point, i32>, points: &Vec<Point>) -> bool {
    circuits.len() == points.len() && circuits.values().copied().counts().len() == 1
}

fn part2() {
    let points = parse_input();
    for point in &points {
        println!("{}", point);
    }

    let mut pq = PriorityQueue::<_, _>::new();
    for a in 0..(points.len()-1) {
        for b in (a+1)..points.len() {
            let dist = points[a].dist(&points[b]);
            pq.push((points[a], points[b]), Reverse(dist));
        }
    }

    let mut circuits: HashMap<Point, i32> = HashMap::new();

    let mut result = OrderedFloat(0.0);

    let mut i = 0;
    while !is_one_circuit(&circuits, &points) {
        let ((a, b), _) = pq.pop().unwrap();
        if let Some(&prev_i) = circuits.get(&a) {
            for (_, j) in &mut circuits {
                if *j == prev_i {
                    *j = i;
                }
            }
        }
        if let Some(&prev_i) = circuits.get(&b) {
            for (_, j) in &mut circuits {
                if *j == prev_i {
                    *j = i;
                }
            }
        }
        circuits.insert(a, i);
        circuits.insert(b, i);
        println!("Connect {} to {}", &a, &b);
        result = a.x * b.x;
        i +=1;
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
