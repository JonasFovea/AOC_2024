use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
const TEST_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[derive(Clone, Copy, Debug)]
enum Field {
    Free,
    Wall,
    Exit,
    Start,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_coords(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn cost_move(&self, other: &Direction) -> usize {
        match self {
            Direction::North => match other {
                Direction::North => 1,
                Direction::East => 1001,
                Direction::South => 2001,
                Direction::West => 1001,
            },
            Direction::East => match other {
                Direction::North => 1001,
                Direction::East => 1,
                Direction::South => 1001,
                Direction::West => 2001,
            },
            Direction::South => match other {
                Direction::North => 2001,
                Direction::East => 1001,
                Direction::South => 1,
                Direction::West => 1001,
            },
            Direction::West => match other {
                Direction::North => 1001,
                Direction::East => 2001,
                Direction::South => 1001,
                Direction::West => 1,
            },
        }
    }
}

const NEIGHBORS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn parse_maze<R: BufRead>(reader: R) -> Vec<Vec<Field>> {
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Field::Wall,
                    'E' => Field::Exit,
                    'S' => Field::Start,
                    _ => Field::Free,
                })
                .collect()
        })
        .collect()
}

fn find_start(maze: &Vec<Vec<Field>>) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if let Field::Start = field {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn find_cheapest_path_to_exit(
    maze: &Vec<Vec<Field>>,
    pos: (usize, usize),
    direction: Direction,
    path: HashSet<(usize, usize)>,
    current_cost: usize,
    cost_map: &mut Vec<Vec<usize>>,
) -> Option<usize> {
    let mut new_path = path.clone();
    new_path.insert(pos);
    let (x, y) = pos;

    if cost_map[y][x] <= current_cost {
        return None;
    } else {
        cost_map[y][x] = current_cost;
    }

    match maze[y][x] {
        Field::Exit => return Some(0),
        Field::Wall => return None,
        _ => {}
    }

    let mut path_lengths = vec![];
    for neighbor in NEIGHBORS.iter() {
        let (dx, dy) = neighbor.as_coords();
        let new_pos = (x as isize + dx, y as isize + dy);
        if new_pos.0 < 0
            || new_pos.0 >= maze[0].len() as isize
            || new_pos.1 < 0
            || new_pos.1 >= maze.len() as isize
        {
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if path.contains(&new_pos) {
            continue;
        }
        if let Some(length) = find_cheapest_path_to_exit(
            maze,
            new_pos,
            *neighbor,
            new_path.clone(),
            current_cost + neighbor.cost_move(&direction),
            cost_map,
        ) {
            path_lengths.push(length + neighbor.cost_move(&direction));
        }
    }
    if !path_lengths.is_empty() {
        return Some(*path_lengths.iter().min().unwrap());
    }

    None
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let maze = parse_maze(reader);
        let mut cost_map = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
        let start = find_start(&maze);
        let path = find_cheapest_path_to_exit(&maze, start, Direction::East, HashSet::new(), 0, &mut cost_map);
        Ok(path.unwrap())
    }

    assert_eq!(7036, time_snippet!(part1(BufReader::new(TEST_1.as_bytes()))?));
    assert_eq!(11048, time_snippet!(part1(BufReader::new(TEST_2.as_bytes()))?));
    println!("Tests passed");

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
