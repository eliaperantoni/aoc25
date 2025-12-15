use std::{env::args, io::stdin};

use itertools::Itertools;
use regex::Regex;

// (0,0) (0,1) (0,2)
// (1,0) (1,1) (1,2)
// (2,0) (2,1) (2,2)
type ShapeT = [[bool; 3]; 3];

const N_SHAPES: usize = 6;

#[derive(Debug)]
struct Shape {
    index: usize,
    shape: ShapeT,
}

impl Shape {
    fn area(&self) -> usize {
        let mut area = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.shape[i][j] {
                    area += 1;
                }
            }
        }
        area
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    must_fit_shapes: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.width * self.length
    }
}

#[derive(Debug, Default)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_input() -> Input {
    let mut input = Input::default();

    let lines = stdin().lines().map(|l| l.unwrap()).collect_vec();

    for i in 0usize..N_SHAPES {
        let mut shape = ShapeT::default();
        for j in 0usize..3 {
            for k in 0usize..3 {
                if lines[5*i+1+j].as_bytes()[k] == b'#' {
                    shape[j][k] = true;
                }
            }

        }

        let shape = Shape {
            index: i,
            shape,
        };
        input.shapes.push(shape);
    }

    let wh_re = Regex::new(r#"^(\d+)x(\d+)"#).unwrap();
    for i in 5*N_SHAPES..lines.len() {
        let m = wh_re.captures(&lines[i]).unwrap();
        let width: usize = m.get(1).unwrap().as_str().parse().unwrap();
        let length: usize = m.get(2).unwrap().as_str().parse().unwrap();

        let mut must_fit_shapes = vec![];
        
        for e in lines[i].trim().split(":").collect_vec()[1].split_ascii_whitespace() {
            let e: usize = e.parse().unwrap();
            must_fit_shapes.push(e); 
        }

        let region = Region {
            width,
            length,
            must_fit_shapes,
        };
        input.regions.push(region);
    }
    
    input
}

fn part1() {
    let input = parse_input();
    println!("Shapes:");
    for i in 0..N_SHAPES {
        println!("{:?} area = {}", &input.shapes[i], input.shapes[i].area());
    }
    println!();
    println!("Regions:");
    for i in 0..input.regions.len() {
        println!("{:?}", &input.regions[i]);
    }
    println!();

    let mut ok_regions = 0;

    for region in &input.regions {
        let mut coverage = 0;

        for i in 0..N_SHAPES {
            coverage += input.shapes[i].area() * region.must_fit_shapes[i];
        }

        if coverage <= region.area() {
            ok_regions += 1;
        }
    }

    println!("{}", ok_regions);
}

fn part2() {

}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
