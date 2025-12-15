use std::{char::MAX, collections::HashMap, env::args, io::stdin};

use itertools::Itertools;
use regex::Regex;
use z3::{Optimize, SatResult, Solver, ast::{Array, Int}};

#[derive(Debug, Clone)]
struct Machine {
    target: i64,
    buttons: Vec<i64>,
}

fn parse_input() -> Vec<Machine> {
    let mut machines = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let pieces = line.split(" ").collect_vec();

        let mut target_str = pieces[0].trim_matches(|c| c == '[' || c == ']');
        let n_lights = target_str.len() as i64;
        let mut target = 0i64;
        while !target_str.is_empty() {
            let b = target_str.as_bytes();
            if b[0] == b'#' {
                target += 1;
            }
            target_str = &target_str[1..];
            if !target_str.is_empty() {
                target <<= 1;
            }
        }

        let mut buttons = vec![];
        for piece in &pieces[1..pieces.len() - 1] {
            let piece = piece.trim_matches(|c| c == '(' || c == ')');
            let mut button = 0i64;
            for digit in piece.split(",") {
                let light_toggled = digit.parse::<i64>().unwrap();
                button += 1 << (n_lights - light_toggled - 1);
            }
            buttons.push(button);
        }

        let machine = Machine { target, buttons };
        machines.push(machine);
    }
    machines
}

fn min_button_presses_to_turn_on_machine(machine: &Machine) -> i64 {
    recurse(machine.target, 0, &machine.buttons[..]).unwrap()
}

fn recurse(t: i64, presses: i64, buttons: &[i64]) -> Option<i64> {
    // println!("{} {} {:?}", t, presses, buttons);
    if t == 0 {
        return Some(presses);
    }
    if buttons.is_empty() {
        return None;
    }
    let if_ye_press = recurse(t ^ buttons[0], presses + 1, &buttons[1..]);
    let if_no_press = recurse(t, presses, &buttons[1..]);
    match (if_ye_press, if_no_press) {
        (Some(if_ye_press), None) => Some(if_ye_press),
        (None, Some(if_no_press)) => Some(if_no_press),
        (None, None) => None,
        (Some(if_ye_press), Some(if_no_press)) => Some(i64::min(if_ye_press, if_no_press)),
    }
}

fn part1() {
    let machines = parse_input();
    for machine in &machines {
        println!("{:?}", machine);
    }
    // dbg!(min_button_presses_to_turn_on_machine(&machines[1]));
    let mut result = 0i64;
    for machine in &machines {
        result += min_button_presses_to_turn_on_machine(machine);
    }
    println!("{}", result);
}

const MAX_N: usize = 10;

#[derive(Debug)]
struct MachinePart2 {
    joltage_requirements: [i64; MAX_N],
    buttons: Vec<[i64; MAX_N]>,
}

fn parse_input_part2() -> Vec<MachinePart2> {
    let mut machines = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let pieces = line.split(" ").collect_vec();

        let mut joltage_requirements = pieces[pieces.len() - 1]
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        assert!(joltage_requirements.len() <= MAX_N);
        while joltage_requirements.len() < MAX_N {
            joltage_requirements.push(0);
        }
        let joltage_requirements: [i64; MAX_N] = joltage_requirements.try_into().unwrap();

        let mut buttons = vec![];
        for piece in &pieces[1..pieces.len() - 1] {
            let mut button = [0; MAX_N];
            for digit in piece.trim_matches(|c| c == '(' || c == ')').split(",") {
                let digit = digit.parse::<usize>().unwrap();
                button[digit] += 1;
            }
            buttons.push(button);
        }

        let machine = MachinePart2 {
            joltage_requirements,
            buttons,
        };
        machines.push(machine);
    }
    machines
}

type MEMO = HashMap<[i64; MAX_N], Option<i64>>;

fn recurse_part2(
    j_req: &[i64; MAX_N],
    current: [i64; MAX_N],
    buttons: &Vec<[i64; MAX_N]>,
    memo: &mut MEMO,
) -> Option<i64> {
    if let Some(&memoized) = memo.get(&current) {
        return memoized;
    }
    if j_req == &current {
        memo.insert(current, Some(0));
        return Some(0);
    }
    for i in 0..MAX_N {
        if current[i] > j_req[i] {
            memo.insert(current, None);
            return None;
        }
    }
    let mut best = None;
    for button in buttons {
        let mut current_cp = current;
        for i in 0..MAX_N {
            current_cp[i] += button[i];
        }
        let contender = recurse_part2(j_req, current_cp, buttons, memo);
        match (best, contender) {
            (None, Some(contender)) => best = Some(contender),
            (Some(best_val), Some(contender)) if contender < best_val => best = Some(contender),
            _ => (),
        };
    }
    let result = best.map(|best| best + 1);
    memo.insert(current, result);
    result
}

fn solve(machine: &MachinePart2) -> i64 {
    let n_buttons = machine.buttons.len();

    let mut button_presses = vec![];
    for i in 0..n_buttons {
        button_presses.push(
            Int::fresh_const(&format!("button_presses_{i}"))
        );
    }

    let opt = Optimize::new();

    for i in 0..n_buttons {
        opt.assert(&button_presses[i].ge(0));
    }

    for i in 0..MAX_N {
        let mut combination = (&button_presses[0] * machine.buttons[0][i]) + (&button_presses[1] * machine.buttons[1][i]);
        for j in 2..n_buttons {
            combination += &button_presses[j] * machine.buttons[j][i];
        }
        opt.assert(&combination.eq(machine.joltage_requirements[i]));
    }

    let mut cost = &button_presses[0] + &button_presses[1];
    for i in 2..n_buttons {
        cost = cost + &button_presses[i];
    }

    opt.minimize(&cost);

    assert_eq!(opt.check(&[]), SatResult::Sat);

    let model = opt.get_model().unwrap();

    let mut result = 0;
    for i in 0..n_buttons {
        let presses = model.eval(&button_presses[i], false).unwrap().as_i64().unwrap();
        result += presses;
    }

    println!("{}", result);

    result
}

fn part2() {
    let machines = parse_input_part2();
    for machine in &machines {
        println!("{:?}", machine);
    }

    let mut result = 0;
    for machine in machines {
        result += solve(&machine);
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
