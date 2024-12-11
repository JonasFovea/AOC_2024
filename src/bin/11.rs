use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

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

fn blink_vec(numbers: &[usize]) -> Vec<usize> {
    let mut res_vec: Vec<usize> = Vec::with_capacity(numbers.len());
    for num in numbers {
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

fn blink_deep(num: usize, depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(num, depth)) {
        return *res;
    }

    match blink(num) {
        BlinkResult::One(a) => {
            let res = blink_deep(a, depth - 1, cache);
            cache.insert((num, depth), res);
            res
        }
        BlinkResult::Two(a, b) => {
            let res = blink_deep(a, depth - 1, cache) + blink_deep(b, depth - 1, cache);
            cache.insert((num, depth), res);
            res
        }
    }
}

fn next_fitting_multiple(number: usize, base: usize) -> usize {
    if number % base == 0 {
        return number;
    }

    let quotient = number / base;
    let multiple = (quotient + 1) * base;

    multiple
}

const MAX_NUM_THREADS: usize = 8;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().unwrap()?;
        let numbers: Vec<_> = line.split(' ').map(|x| x.parse().unwrap()).collect();

        let mut count = 0;
        for num in numbers {
            let mut stones = vec![num];
            for _ in 0..25 {
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

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().unwrap()?;
        let mut numbers: Vec<_> = line.split(' ').map(|x| x.parse().unwrap()).collect();

        let num_count = numbers.len();

        let num_threads: usize = if num_count < MAX_NUM_THREADS {
            num_count
        } else {
            MAX_NUM_THREADS
        };

        let optimal_queue_size = next_fitting_multiple(num_count, num_threads);
        let num_per_thread = optimal_queue_size / num_threads;

        let mut queues = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let mut tasks = Vec::with_capacity(num_per_thread);
            for j in 0..num_per_thread {
                let idx = (j * num_threads) + i;
                if idx >= num_count {
                    break;
                }
                tasks.push(numbers[idx]);
            }
            queues.push(tasks);
        }


        let threads: Vec<_> = (0..num_threads)
            .map(|i| {
                thread::spawn({
                    let queue = queues[i].clone();
                    move || {
                        let mut cache = HashMap::new();
                        let mut count = 0;
                        for (_i, num) in queue.iter().enumerate() {
                            // println!(
                            //     "Thread {:?}, blinking number {}/{}: {}",
                            //     thread::current().id(),
                            //     _i+1,
                            //     queue.len(),
                            //     *num
                            // );
                            let blink_res = blink_deep(*num, 75, &mut cache);
                            count += blink_res;
                        }
                        count
                    }
                })
            })
            .collect();

        let mut count = 0;
        for handle in threads {
            count += handle.join().unwrap();
        }

        Ok(count)
    }

    println!("Part 2 on test input: {}", part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
