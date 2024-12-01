use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::isize;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        let re = Regex::new(r"(\d+)\s+(\d+)")?;

        for line in reader.lines() {
            let line = line?;
            let cap = re.captures(&line).unwrap();
            let l_val = isize::from_str_radix(cap.get(1).unwrap().as_str(), 10)?;
            let r_val = isize::from_str_radix(cap.get(2).unwrap().as_str(), 10)?;
            left.push(l_val);
            right.push(r_val);
        }

        left.sort();
        right.sort();

        let answer = left
            .iter()
            .zip(right)
            .map(|(a, b)| { isize::abs(*a - b) } as usize)
            .sum();

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

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
