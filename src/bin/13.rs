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

fn win_prize(
    claw_machine: &ClawMachine,
    max_pushes: usize,
) -> Option<(usize, usize)> {

    let mut solutions = Vec::new();

    'a_loop: for a in 0..=max_pushes {
        if a*claw_machine.button_a.0 > claw_machine.prize.0 {
            break;
        }

        for b in 0..=max_pushes {
            let x = claw_machine.button_a.0 * a + claw_machine.button_b.0 * b;
            let y = claw_machine.button_a.1 * a + claw_machine.button_b.1 * b;

            if x>claw_machine.prize.0 || y>claw_machine.prize.1 {
                continue 'a_loop;
            }


            if (x, y) == claw_machine.prize {
                solutions.push((a,b, 3*a+b));
                continue 'a_loop;
            }

        }
    }

    if !solutions.is_empty() {
        solutions.sort_by_key(|x| x.2);
        return Some((solutions[0].0, solutions[0].1));
    }

    None
}

fn parse_claw_machines<R: BufRead>(reader: R) -> Vec<ClawMachine> {
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
        let p_x = prize.get(1).unwrap().as_str().parse().unwrap();
        let p_y = prize.get(2).unwrap().as_str().parse().unwrap();

        claw_machines.push(ClawMachine {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (p_x, p_y),
        });
    }

    claw_machines
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let claw_machines = parse_claw_machines(reader);

        let mut total_cost = 0;
        for claw_machine in claw_machines {
            let res = win_prize(&claw_machine, 100);
            if let Some((a, b)) = res {
                total_cost += 3 * a + b;
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
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
