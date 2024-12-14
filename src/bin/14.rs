use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

const TEST_SIZE: (isize, isize) = (11, 7);

const ACTUAL_SIZE: (isize, isize) = (101, 103);

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    limits: (isize, isize),
}

impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize), limits: (isize, isize)) -> Self {
        Self {
            position,
            velocity,
            limits,
        }
    }

    fn from_str(s: &str, limits: (isize, isize)) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let position = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let velocity = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        Self::new(position, velocity, limits)
    }

    fn position_at(&self, time: isize) -> (isize, isize) {
        let mut new_pos = (
            (self.position.0 + self.velocity.0 * time) % self.limits.0,
            (self.position.1 + self.velocity.1 * time) % self.limits.1,
        );
        if new_pos.0 < 0 {
            new_pos.0 += self.limits.0;
        }
        if new_pos.1 < 0 {
            new_pos.1 += self.limits.1;
        }
        new_pos
    }
}

fn quadrant_of_pos(pos: (isize, isize), limits: (isize, isize)) -> usize {
    match (
        pos.0 < limits.0 / 2,
        pos.0 > limits.0 / 2,
        pos.1 < limits.1 / 2,
        pos.1 > limits.1 / 2,
    ) {
        (false, false, _, _) => 0,
        (_, _, false, false) => 0,
        (true, _, true, _) => 1,
        (false, _, true, _) => 2,
        (false, _, false, _) => 3,
        (true, _, false, _) => 4,
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, field_size: (isize, isize)) -> Result<usize> {
        let mut answers: [usize; 5] = [0, 0, 0, 0, 0];
        for line in reader.lines() {
            let line = line?;
            let robot = Robot::from_str(&line, field_size);
            let pos_100 = robot.position_at(100);
            answers[quadrant_of_pos(pos_100, field_size)] += 1;
        }

        let answer: usize = answers[1..]
            .iter()
            .filter(|x| (**x) != 0)
            .map(|x1| *x1)
            .reduce(|a, b| a * b)
            .unwrap();
        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), TEST_SIZE)?);
    println!("Test passed");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, ACTUAL_SIZE)?);
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
