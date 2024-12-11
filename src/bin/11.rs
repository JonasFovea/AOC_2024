use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn split_number(num: usize, num_digits: usize) -> (usize, usize) {
    let ten_pow = 10_usize.pow((num_digits / 2) as u32);
    let left = num / ten_pow;
    let right = num % ten_pow;

    // println!("Splitting {}: {num} / {ten_pow} = {} and {num} % {ten_pow} = {}", num, left, right);
    (left, right)
}

fn count_digits(num: usize) -> usize {
    let mut digit_count = 1;
    let mut num_copy = num;
    while num_copy > 9 {
        digit_count += 1;
        num_copy /= 10;
    }
    digit_count
}

enum BlinkResult {
    One(usize),
    Two(usize, usize),
}

fn blink(num: usize) -> BlinkResult {
    let d_cont = count_digits(num);
    if num == 0 {
        BlinkResult::One(1)
    } else if d_cont % 2 == 0 {
        let (left, right) = split_number(num, d_cont);
        BlinkResult::Two(left, right)
    } else {
        BlinkResult::One(num * 2024)
    }
}

fn blink_vec(numbers: &[usize])-> Vec<usize>{
    let mut res_vec: Vec<usize> = Vec::with_capacity(numbers.len());
    for num in numbers{
        match blink(*num) {
            BlinkResult::One(new_num) => {
                res_vec.push(new_num);
            }
            BlinkResult::Two(a, b) => {
                res_vec.push(a);
                res_vec.push(b);
            }
        }
    }
    res_vec
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().unwrap()?;
        let numbers: Vec<_> = line.split(' ').map(|x| x.parse().unwrap()).collect();


        let mut count = 0;
        for num in numbers{
            let mut stones = vec![num];
            for _ in 0..25{
                stones = blink_vec(&stones);
            }
            count += stones.len();
        }


        Ok(count)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

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

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
