use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn process_input<R: BufRead>(reader: R) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut read_pairs = true;

    let re_pair = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let re_update = Regex::new(r"(\d+)").unwrap();

    let mut pair_map = HashMap::new();
    let mut update_list = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if read_pairs {
            if line.len() == 0 {
                read_pairs = false;
                continue;
            }

            let pair = re_pair.captures(&line).unwrap();
            let a = usize::from_str_radix(pair.get(1).unwrap().as_str(),10).unwrap();
            let b = usize::from_str_radix(pair.get(2).unwrap().as_str(),10).unwrap();
            if pair_map.get(&a).is_none(){
                pair_map.insert(a, vec![b]);
            }else {
                pair_map.get_mut(&a).unwrap().push(b);
            }
        } else {
            let mut update = Vec::new();
            for num in re_update.captures_iter(&line) {
                update.push(usize::from_str_radix(num.extract::<1>().0, 10).unwrap())
            }
            update_list.push(update);
        }
    }

    (pair_map, update_list)
}

fn is_update_correct(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    let mut idx_map = HashMap::with_capacity(update.len());
    for (idx, val) in update.iter().enumerate() {
        idx_map.insert(*val, idx);
    }


    for (idx, num) in update.iter().enumerate() {
        if let Some(other_vec) = rules.get(num) {
            for other in other_vec {
                if let Some(other_idx) = idx_map.get(other) {
                    if idx >= *other_idx {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (ordering, pages) = process_input(reader);
        let correct_pages = pages.iter().filter(|x| {is_update_correct(&ordering, *x)}).collect::<Vec<_>>();
        let middle_sum = correct_pages.iter().map(|x1| {x1[x1.len()/2]}).sum();
        Ok(middle_sum)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

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
