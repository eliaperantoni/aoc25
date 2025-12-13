use std::{env::args, fmt::Display, fs, num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split(",").collect();
        if pieces.len() != 2 {
            return Err("need 2 pieces".to_string());
        }
        Ok(Point {
            x: pieces[0]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            y: pieces[1]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
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

fn area(a: Point, b: Point) -> i64 {
    width(a, b) * height(a, b)
}

fn part1() {
    let points = parse_input();

    let mut best = 1;

    for i in 0..(points.len() - 1) {
        for j in (i + 1)..points.len() {
            best = best.max(area(points[i], points[j]))
        }
    }

    println!("{}", best);
}

// x ->>

// y
// |
// |
// v
fn is_valid_rectangle(points: &Vec<Point>, i: usize, j: usize) -> bool {
    let point_i = points[i];
    let point_j = points[j];

    let x_min = i64::min(point_i.x, point_j.x);
    let x_max = i64::max(point_i.x, point_j.x);

    let y_min = i64::min(point_i.y, point_j.y);
    let y_max = i64::max(point_i.y, point_j.y);

    for line in lines(points) {
        if is_line_horizontal(line) {
            assert_eq!(line.0.y, line.1.y);
            let line_y = line.0.y;
            if line_y <= y_min || line_y >= y_max {
                continue;
            }
            let line_x_left = line.0.x.min(line.1.x);
            let line_x_right = line.0.x.max(line.1.x);
            if line_x_right <= x_min {
                continue;
            }
            if line_x_left >= x_max {
                continue;
            }
            return false;
        } else {
            assert_eq!(line.0.x, line.1.x);
            let line_x = line.0.x;
            if line_x <= x_min || line_x >= x_max {
                continue;
            }
            let line_y_up = line.0.y.min(line.1.y);
            let line_y_down = line.0.y.max(line.1.y);
            if line_y_down <= y_min {
                continue;
            }
            if line_y_up >= y_max {
                continue;
            }
            return false;
        }
    }

    true
}

fn is_line_horizontal(line: (Point, Point)) -> bool {
    line.0.x != line.1.x
}

fn lines(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut lines = vec![];
    for i in 0..points.len() {
        let i_next = (i + 1) % points.len();
        lines.push((points[i], points[i_next]));
    }
    lines
}

fn width(a: Point, b: Point) -> i64 {
    (a.x - b.x).abs() + 1
}

fn height(a: Point, b: Point) -> i64 {
    (a.y - b.y).abs() + 1
}

fn draw_svg(points: &Vec<Point>, rect: Option<(Point, Point)>) -> String {
    dbg!(rect);
    let points_sequence = points
        .into_iter()
        .map(|p| format!("{},{}", p.x, p.y))
        .join(" ");
    let points_str = points
        .iter()
        .map(|p| format!(r#"<circle r="100" cx="{}" cy="{}" fill="red" />"#, p.x, p.y))
        .join("\n");
    let mut rect_str = String::new();
    if let Some(rect) = rect {
        let (a, b) = rect;
        let c = Point { x: a.x, y: b.y };
        let d = Point { x: b.x, y: a.y };
        let x = [a.x, b.x, c.x, d.x].into_iter().min().unwrap();
        let y = [a.y, b.y, c.y, d.y].into_iter().min().unwrap();
        rect_str = format!(
            r#"<rect width="{}" height="{}" x="{}" y="{}" fill="blue" opacity="0.5" />"#,
            (a.x - b.x).abs(),
            (a.y - b.y).abs(),
            x,
            y
        );
    }
    return format!(
        r#"<svg viewBox="0 0 100000 100000" xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="white"/>
    <polygon points="{points_sequence}" fill="green"/>
    {points_str}
    {rect_str}
</svg>"#
    );
}

fn part2() {
    let points = parse_input();

    let mut best = 1;
    let mut best_idxs = None;

    for i in 0..(points.len()-1) {
        for j in (i+1)..points.len() {
            // println!("{} --- {}", points[i], points[j]);
            // println!("valid? {}", is_valid_rectangle(&points, i, j));
            // println!("area = {}", area(points[i], points[j]));
            if !is_valid_rectangle(&points, i, j) {
                continue;
            }
            let contender = area(points[i], points[j]);
            if contender > best {
                best = contender;
                best_idxs = Some((points[i], points[j]));
            }
        }
    }

    let svg = draw_svg(&points, best_idxs);
    fs::write("./day9.svg", svg).unwrap();

    println!("{}", best);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
