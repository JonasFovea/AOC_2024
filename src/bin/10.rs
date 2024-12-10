use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";


const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn is_in_field(pos: (isize, isize), field_size: (isize, isize)) -> bool {
    pos.0 >= 0 && pos.0 < field_size.0 && pos.1 >= 0 && pos.1 < field_size.1
}

fn is_correct_next_char(current: char, next: char) -> bool {
    match current {
        '0' => next == '1',
        '1' => next == '2',
        '2' => next == '3',
        '3' => next == '4',
        '4' => next == '5',
        '5' => next == '6',
        '6' => next == '7',
        '7' => next == '8',
        '8' => next == '9',
        '9' => next == '0',
        _ => false,
    }
}

fn count_complete_trails(
    field: &Vec<Vec<char>>,
    start: (isize, isize),
    mut visited: HashSet<(isize, isize)>
) -> HashSet<(isize, isize)> {
    if !visited.insert(start) {
        return HashSet::new();
    }

    if field[start.0 as usize][start.1 as usize] == '9' {
        let mut top_set = HashSet::new();
        top_set.insert(start);
        return top_set;
    }

    let mut top_set = HashSet::new();
    for n in NEIGHBORS {
        let next = (start.0 + n.0, start.1 + n.1);
        if is_in_field(next, (field.len() as isize, field[0].len() as isize))
            && is_correct_next_char(
                field[start.0 as usize][start.1 as usize],
                field[next.0 as usize][next.1 as usize],
            )
        {
            let path_set =count_complete_trails(field, next, visited.clone());
            top_set.extend(path_set);
        }
    }
    top_set
}

fn list_starts(field: &Vec<Vec<char>>) -> Vec<(isize, isize)> {
    let mut starts = Vec::new();
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == '0' {
                starts.push((row as isize, col as isize));
            }
        }
    }
    starts
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let field = reader
            .lines()
            .map(|line| line.map(|l| l.chars().collect::<Vec<_>>()))
            .collect::<Result<Vec<_>, _>>()?;

        let starts = list_starts(&field);

        let mut sum_score = 0;
        for start in starts {
            let trail_score = count_complete_trails(&field, start, HashSet::with_capacity(10)).len();
            sum_score += trail_score;
        }
        Ok(sum_score)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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
