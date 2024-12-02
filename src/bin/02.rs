use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn check_line(nums: Vec<&str>) -> Result<usize>{
    let mut dir = 0;
    for (a, b) in nums.iter().zip(nums.iter().skip(1)) {
        let dist = isize::from_str_radix(a, 10)? - isize::from_str_radix(b, 10)?;
        if dir == 0 {
            dir = if dist > 0 { 1 } else { -1 };
        } else {
            if dir * dist < 0 {
                return Ok(0);
            }
        }
        if dist.abs() < 1 || dist.abs() > 3 {
            return Ok(0);
        }
    }
    Ok(1)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader.lines().map(|line| {
            let line_nums = line.unwrap();
            let nums: Vec<_> = line_nums.split(" ").collect();
            check_line(nums).unwrap()
        }).sum();


        Ok(safe_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader.lines().map(|line| {
            let line_nums = line.unwrap();
            let nums: Vec<_> = line_nums.split(" ").collect();

            for i in 0..nums.len(){
                let mut local_nums = nums.clone();
                local_nums.remove(i);
                if check_line(local_nums).unwrap() == 1{
                    return 1;
                }
            }
            0
        }).sum();


        Ok(safe_count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
