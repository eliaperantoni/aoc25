use std::{env::args, io::Read};

fn count_rolls_of_paper_around(lines: &[String], m: usize, n: usize, i: usize, j: usize) -> usize {
    let mut count = 0;

    for di in [-1isize, 0, 1].iter() {
        for dj in [-1isize, 0, 1].iter() {
            if *di == 0 && *dj == 0 {
                continue;
            }
            let ni = i as isize + *di;
            let nj = j as isize + *dj;
            if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                if char_at(lines, ni as usize, nj as usize) == b'@' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn char_at(lines: &[String], i: usize, j: usize) -> u8 {
    lines[i].as_bytes()[j]
}

fn remove_paper(lines: &mut Vec<String>, i: usize, j: usize) {
    lines[i] = format!("{}.{}", &lines[i][0..j], &lines[i][j+1..lines[i].len()]);
}

fn debug_print(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

fn part1() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();

    let m = lines.len();
    let n = lines[0].len();

    let mut result = 0;

    for i in 0..m {
        for j in 0..n {
            if char_at(&lines, i, j) == b'@' && count_rolls_of_paper_around(&lines, m, n, i, j) < 4 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn part2() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines: Vec<_> = input.lines().map(str::to_string).collect();

    let m = lines.len();
    let n = lines[0].len();

    let mut result = 0;

    let mut is_first_iteration = true;
    let mut did_remove_anything = false;

    while is_first_iteration || did_remove_anything {
        is_first_iteration = false;
        did_remove_anything = false;

        for i in 0..m {
            for j in 0..n {
                if char_at(&lines, i, j) == b'@' && count_rolls_of_paper_around(&lines, m, n, i, j) < 4 {
                    remove_paper(&mut lines, i, j);
                    result += 1;
                    did_remove_anything = true;
                }
            }
        }
    }

    debug_print(&lines);

    println!("{}", result);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
