use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_coords(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Marker {
    position: (usize, usize),
    direction: Direction,
}

impl Marker {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

fn count_individual_positions(walked_fields: &HashSet<Marker>) -> usize {
    filter_positions(walked_fields).len()
}

fn filter_positions(walked_fields: &HashSet<Marker>) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    for marker in walked_fields.iter() {
        positions.insert(marker.position);
    }
    positions
}


fn walk_field(start: (usize, usize), field: &Vec<Vec<char>>) -> (usize, bool, HashSet<Marker>) {
    let mut walked_fields = HashSet::new();
    let mut current_field = start;
    let mut current_direction = Direction::Up;
    loop {
        if !walked_fields.insert(Marker::new(current_field, current_direction)) {
            return (count_individual_positions(&walked_fields), true, walked_fields);
        }

        let next_field = (
            current_field.0 as isize + current_direction.get_coords().0,
            current_field.1 as isize + current_direction.get_coords().1,
        );

        if next_field.0 < 0
            || next_field.0 >= field.len() as isize
            || next_field.1 < 0
            || next_field.1 >= field[next_field.0 as usize].len() as isize
        {
            break;
        }

        if field[next_field.0 as usize][next_field.1 as usize] == '#' {
            current_direction = current_direction.turn();
            continue;
        }
        current_field = (next_field.0 as usize, next_field.1 as usize);
    }
    (count_individual_positions(&walked_fields), false, walked_fields)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let field = reader
            .lines()
            .map(|x| x.unwrap().chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut start = None;
        for (row_idx, row) in field.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    start = Some((row_idx, col_idx));
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }

        let answer = walk_field(start.unwrap(), &field);
        Ok(answer.0)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let field = reader
            .lines()
            .map(|x| x.unwrap().chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut start = None;
        for (row_idx, row) in field.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    start = Some((row_idx, col_idx));
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }

        let mut loop_count = 0;
        let (_, _, walked_fields) = walk_field(start.unwrap(), &field);
        for position in filter_positions(&walked_fields).iter() {
            if *position == start.unwrap() {
                continue;
            }
            let mut test_field = field.clone();
            test_field[position.0][position.1] = '#';
            let (_, looped, _) = walk_field(start.unwrap(), &test_field);
            if looped {
                loop_count += 1;
            }
        }

        Ok(loop_count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
