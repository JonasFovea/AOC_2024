use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

fn find_start_end(maze: &Vec<Vec<Field>>) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if let Field::Start = field {
                return (x, y);
            }
        }
    }
    panic!("No start and end found");
}

fn find_shortest_path(
    maze: &Vec<Vec<Field>>,
    pos: (usize, usize),
    path: &Vec<(usize, usize)>,
    cost_map: &mut Vec<Vec<usize>>,
) -> Option<Vec<(usize, usize)>> {
    let (x, y) = pos;
    if x >= maze[0].len() || y >= maze.len() {
        return None;
    }
    if maze[y][x] == Field::Wall {
        return None;
    }

    let mut path = path.clone();
    path.push(pos);

    if maze[y][x] == Field::Exit {
        return Some(path);
    }
    if path.len() >= cost_map[y][x] {
        return None;
    }
    cost_map[y][x] = path.len();

    let mut shortest_paths = Vec::new();
    for dir in NEIGHBORS.iter() {
        let (dx, dy) = dir.as_coords();
        let new_pos = (x as isize + dx, y as isize + dy);
        if new_pos.0 >= 0 && new_pos.1 >= 0 {
            if let Some(new_path) = find_shortest_path(
                maze,
                (new_pos.0 as usize, new_pos.1 as usize),
                &path,
                cost_map,
            ) {
                shortest_paths.push(new_path);
            }
        }
    }

    if shortest_paths.is_empty() {
        None
    } else {
        shortest_paths.sort_by_key(|path| path.len());
        Some(shortest_paths[0].clone())
    }
}

fn find_cheats(maze: &Vec<Vec<Field>>, path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut cheats = HashSet::new();
    for (x, y) in path.iter() {
        if *x > 1 && maze[*y][x - 1] == Field::Wall && maze[*y][x - 2] == Field::Free {
            cheats.insert((x - 1, *y));
        }
        if *x < maze[0].len() - 2
            && maze[*y][x + 1] == Field::Wall
            && maze[*y][x + 2] == Field::Free
        {
            cheats.insert((x + 1, *y));
        }
        if *y > 1 && maze[y - 1][*x] == Field::Wall && maze[y - 2][*x] == Field::Free {
            cheats.insert((*x, y - 1));
        }
        if *y < maze.len() - 2 && maze[y + 1][*x] == Field::Wall && maze[y + 2][*x] == Field::Free {
            cheats.insert((*x, y + 1));
        }
    }
    cheats.iter().copied().collect()
}

fn path_up_to_cheat(path: &Vec<(usize, usize)>, cheat: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut new_path = Vec::with_capacity(path.len());
    for pos in path {
        new_path.push(*pos);
        if pos.0 == cheat.0 && (pos.1 == cheat.1 - 1 || pos.1 == cheat.1 + 1) {
            break;
        }
        if pos.1 == cheat.1 && (pos.0 == cheat.0 - 1 || pos.0 == cheat.0 + 1) {
            break;
        }
    }
    new_path
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let maze = parse_maze(reader);
        let (start_x, start_y) = find_start_end(&maze);
        let mut cost_map = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
        let path = find_shortest_path(&maze, (start_x, start_y), &vec![], &mut cost_map).unwrap();
        // println!("Found initial path of length {}", path.len());

        let cheats = find_cheats(&maze, &path);
        // println!("Found {} cheats", cheats.len());

        let mut time_savings = vec![];

        let _num_cheats = cheats.len();
        for (_i, cheat) in cheats.iter().enumerate() {
            let mut cheat_maze = maze.clone();
            cheat_maze[cheat.1][cheat.0] = Field::Free;

            let mut cost_map = cost_map.clone();
            let path_to_cheat = path_up_to_cheat(&path, &cheat);
            let new_path =
                find_shortest_path(&cheat_maze, *cheat, &path_to_cheat, &mut cost_map).unwrap();
            time_savings.push(path.len() - new_path.len());
            // println!("\rTested cheat {:5}/{_num_cheats}", _i);
        }

        let mut save_map: HashMap<usize, usize> = HashMap::new();
        for saving in &time_savings {
            match save_map.get(saving) {
                Some(count) => save_map.insert(*saving, *count + 1),
                None => save_map.insert(*saving, 1),
            };
        }

        // println!("Time savings: {:?}", save_map);

        let mut count_at_least_100 = 0;
        for (k, v) in save_map {
            if k >= 100 {
                count_at_least_100 += v;
            }
        }

        Ok(count_at_least_100)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

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
