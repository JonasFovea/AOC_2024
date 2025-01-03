use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::izip;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn calc_cost(claw_machine: &ClawMachine) -> Option<usize> {
    let a0 = claw_machine.button_a.0 as f64;
    let a1 = claw_machine.button_a.1 as f64;
    let b0 = claw_machine.button_b.0 as f64;
    let b1 = claw_machine.button_b.1 as f64;
    let p0 = claw_machine.prize.0 as f64;
    let p1 = claw_machine.prize.1 as f64;

    let s = (p1 - (a1 * p0) / a0) / ((a1 * b0) / a0 - b1);
    let x = (s * b0 + p0, s * b1 + p1);
    let diff_b = (p0 - x.0, p1 - x.1);

    let num_a = (x.0 / a0).round() as usize;
    let num_b = (diff_b.0 / b0).round() as usize;

    let price_test = (
        claw_machine.button_a.0 * num_a + claw_machine.button_b.0 * num_b,
        claw_machine.button_a.1 * num_a + claw_machine.button_b.1 * num_b,
    );
    if price_test != claw_machine.prize {
        return None;
    }

    let costs = 3 * num_a + num_b;
    Some(costs)
}

fn parse_claw_machines<R: BufRead>(reader: R, prize_offset: usize) -> Vec<ClawMachine> {
    let re_a_button = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_b_button = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut claw_machines = Vec::new();

    let lines: Vec<_> = reader.lines().map(|x| x.unwrap()).collect();

    for (a, b, p) in izip!(
        lines.iter().step_by(4),
        lines.iter().skip(1).step_by(4),
        lines.iter().skip(2).step_by(4)
    ) {
        let button_a = re_a_button.captures(a).unwrap();
        let a_x = button_a.get(1).unwrap().as_str().parse().unwrap();
        let a_y = button_a.get(2).unwrap().as_str().parse().unwrap();

        let button_b = re_b_button.captures(b).unwrap();
        let b_x = button_b.get(1).unwrap().as_str().parse().unwrap();
        let b_y = button_b.get(2).unwrap().as_str().parse().unwrap();

        let prize = re_prize.captures(p).unwrap();
        let p_x: usize = prize.get(1).unwrap().as_str().parse().unwrap();
        let p_y: usize = prize.get(2).unwrap().as_str().parse().unwrap();

        claw_machines.push(ClawMachine {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (p_x + prize_offset, p_y + prize_offset),
        });
    }

    claw_machines
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let claw_machines = parse_claw_machines(reader, 0);

        let mut total_cost = 0;
        for claw_machine in claw_machines {
            if let Some(cost) = calc_cost(&claw_machine) {
                total_cost += cost;
            }
        }

        Ok(total_cost)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let claw_machines = parse_claw_machines(reader, 10000000000000);

        let mut total_cost = 0;
        for claw_machine in claw_machines {
            if let Some(cost) = calc_cost(&claw_machine) {
                total_cost += cost;
            }
        }

        Ok(total_cost)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
