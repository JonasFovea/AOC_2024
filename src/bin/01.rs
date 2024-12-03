use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::HashMap;
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

fn split_lists<R: BufRead>(reader: R) -> Result<(Vec<isize>, Vec<isize>)> {
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

    Ok((left, right))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left, mut right) = split_lists(reader)?;

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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = split_lists(reader)?;
        let mut right_cout_map = HashMap::new();
        for r_val in &right {
            match right_cout_map.get(r_val) {
                Some(count) => {
                    right_cout_map.insert(r_val, count + 1);
                }
                None => {
                    right_cout_map.insert(r_val, 1);
                }
            }
        }

        let mut similarity_score = 0;
        for l_val in left {
            let count = if let Some(c) = right_cout_map.get(&l_val) {
                *c
            } else {
                0
            };
            similarity_score += l_val * count;
        }

        Ok(similarity_score as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
