use core::num;
use std::{env::args, io::Read};

#[derive(Debug)]
struct Column {
    operands: Vec<i64>,
    operator: char,
}

impl Column {
    fn compute(&self) -> i64 {
        match self.operator {
            '+' => self.operands.iter().sum(),
            '*' => self.operands.iter().copied().reduce(|a, b| a * b).unwrap(),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct PuzzleInput {
    columns: Vec<Column>,
}

impl PuzzleInput {
    fn get_cols_widths(lines: &[String]) -> Vec<usize> {
        let last_line = lines.last().unwrap();
        let mut current_col_width = 0usize;
        let mut cols_width = Vec::new();

        let mut i = 0;
        while i < last_line.len() {
            if current_col_width == 0 {
                i += 1;
                current_col_width += 1;
                continue;
            }

            if last_line.as_bytes()[i].is_ascii_whitespace() {
                i += 1;
                current_col_width += 1;
            } else {
                i += 1;
                cols_width.push(current_col_width);
                current_col_width = 1;
            }
        }
        cols_width.push(current_col_width);

        println!("{:?}", cols_width);

        cols_width
    }

    fn parse_part1(lines: &[String]) -> Self {
        let cols_width = Self::get_cols_widths(lines);
        let mut result = Self { columns: Vec::new() };
        
        let mut col_start = 0usize;
        for col_width in &cols_width {
            let mut column = Column {
                operands: Vec::new(),
                operator: ' ',
            };
            for (i, line) in lines.iter().enumerate() {
                if i == lines.len() - 1 {
                    column.operator = line.as_bytes()[col_start] as char;
                } else {
                    let operand: i64 = line[col_start..col_start+col_width].trim().parse().unwrap();
                    column.operands.push(operand);
                }
            }
            result.columns.push(column);
            col_start += col_width;
        }

        result
    }

    fn parse_part2(lines: &[String]) -> Self {
        let lines_except_last = &lines[0..lines.len() - 1];
        let cols_width = Self::get_cols_widths(lines);
        let mut result = Self { columns: Vec::new() };
        
        let mut col_start = 0usize;
        for col_width in cols_width {
            let mut column = Column {
                operands: Vec::new(),
                operator: lines.last().unwrap().as_bytes()[col_start] as char,
            };
            for i in 0..col_width {
                let mut operand_str = String::new();
                for line in lines_except_last {
                    operand_str += &(line.as_bytes()[col_start + i] as char).to_string();
                }
                if operand_str.trim().is_empty() {
                    break;
                }
                let operand: i64 = operand_str.trim().parse().unwrap();
                column.operands.push(operand);
            }
            result.columns.push(column);
            col_start += col_width;
        }

        result
    }
}

fn part1() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();
    let puzzle_input = PuzzleInput::parse_part1(&lines);

    let result: i64 = puzzle_input.columns.into_iter().map(|c| c.compute()).sum();
    println!("{}", result);
}

fn part2() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().map(str::to_string).collect();
    let puzzle_input = PuzzleInput::parse_part2(&lines);

    println!("{:?}", puzzle_input);
    
    let result: i64 = puzzle_input.columns.into_iter().map(|c| c.compute()).sum();
    println!("{}", result);
}

fn main() {
    match args().nth(1).as_deref() {
        Some("1") => return part1(),
        Some("2") => return part2(),
        _ => {}
    }
}
