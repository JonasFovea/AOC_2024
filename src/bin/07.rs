use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn check_slice_add_mul(result: usize, current_result: usize, rest_values: &[usize]) -> bool {
    if rest_values.len() == 0 {
        return result == current_result;
    }

    let next_add = current_result + rest_values[0];
    let next_mul = if current_result != 0 {
        current_result * rest_values[0]
    } else {
        rest_values[0]
    };

    let mut add_res = false;
    let mut mul_res = false;

    if next_add <= result {
        add_res = check_slice_add_mul(result, next_add, &rest_values[1..]);
    }
    if next_mul <= result {
        mul_res = check_slice_add_mul(result, next_mul, &rest_values[1..]);
    }

    add_res || mul_res
}

fn check_slice_add_mul_concat(result: usize, current_result: usize, rest_values: &[usize]) -> bool {
    if rest_values.len() == 0 {
        return result == current_result;
    }

    let next_add = current_result + rest_values[0];
    let next_mul = if current_result != 0 {
        current_result * rest_values[0]
    } else {
        rest_values[0]
    };
    let next_concat: usize = concatenate_integers(current_result, rest_values[0]);


    let mut add_res = false;
    let mut mul_res = false;
    let mut concat_res = false;

    if next_add <= result {
        add_res = check_slice_add_mul_concat(result, next_add, &rest_values[1..]);
    }
    if next_mul <= result {
        mul_res = check_slice_add_mul_concat(result, next_mul, &rest_values[1..]);
    }
    if next_concat <= result {
        concat_res = check_slice_add_mul_concat(result, next_concat, &rest_values[1..]);
    }

    add_res || mul_res || concat_res
}

fn concatenate_integers(a: usize, b: usize) -> usize {
    let mut b_copy = b;
    let mut digits = 0;

    while b_copy > 0 {
        b_copy /= 10;
        digits += 1;
    }

    a * 10_usize.pow(digits) + b
}

fn read_calibrations<R: BufRead>(reader: R) -> Vec<(usize, Vec<usize>)>{
    reader.lines()
        .map(|x| {
            let line = x.unwrap();
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            (result, values)
        })
        .collect()
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let calibrations: Vec<(usize, Vec<usize>)> = read_calibrations(reader);

        let answer = calibrations
            .iter()
            .filter(|(result, values)| check_slice_add_mul(*result, 0, values))
            .map(|(result, _)| *result).sum();

        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let calibrations: Vec<(usize, Vec<usize>)> = read_calibrations(reader);

        let answer = calibrations
            .iter()
            .filter(|(result, values)| check_slice_add_mul_concat(*result, 0, values))
            .map(|(result, _)| *result).sum();

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
