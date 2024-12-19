use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn read_designs<R: BufRead>(reader: R) -> Result<(Vec<String>, Vec<String>)> {
    let mut patterns = Vec::new();
    let mut designs = Vec::new();
    let mut lines = reader.lines();
    let first_line = lines.next().context("No first line")??;
    for pattern in first_line.split(", ") {
        patterns.push(pattern.to_string());
    }
    lines.next(); // Skip empty line
    for line in lines {
        let line = line?;
        designs.push(line);
    }
    Ok((patterns, designs))
}

fn design_matches_patterns(patterns: &[String], design: &str) -> usize {
    let mut count = 0;
    for pat in patterns {
        if pat.len() > design.len() {
            continue;
        }
        if design[0..pat.len()] == *pat {
            if design.len() == pat.len() {
                count += 1;
                continue;
            } else {
                count += design_matches_patterns(patterns, &design[pat.len()..]);
            };
        }
    }
    count
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (patterns, designs) = read_designs(reader)?;

        let valid_designs = designs
            .iter()
            .filter(|d| design_matches_patterns(&patterns, d)>0);

        Ok(valid_designs.count())
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

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
