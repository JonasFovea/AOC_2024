use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn match_and_sum_mul(re: &Regex, input: &str) -> Result<usize> {
    let mut sum = 0;
    for mul in re.captures_iter(&input) {
        let a = usize::from_str_radix(mul.get(1).unwrap().as_str(), 10)?;
        let b = usize::from_str_radix(mul.get(2).unwrap().as_str(), 10)?;
        sum += a * b;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let mut sum = 0;
        for line in reader.lines() {
            sum += match_and_sum_mul(&re, &line?)?;
        }
        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let mut sum = 0;

        let line = reader
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join("");
        let dont_parts: Vec<_> = line.split("don't()").collect();

        // part before any "don't"
        let first = dont_parts[0];
        sum += match_and_sum_mul(&re, first)?;

        for i in 1..dont_parts.len() {
            let do_parts: Vec<_> = dont_parts[i].split("do()").collect();

            // if do() existed
            if do_parts.len() > 1 {
                for j in 1..do_parts.len() {
                    sum += match_and_sum_mul(&re, do_parts[j])?;
                }
            }
        }

        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
