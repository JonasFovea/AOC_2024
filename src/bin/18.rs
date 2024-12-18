use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

const TEST_SIZE: (usize, usize) = (7, 7);
const REAL_SIZE: (usize, usize) = (71, 71);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    Free,
    Wall(usize),
    Exit,
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

fn read_input<R: BufRead>(reader: R, max_byte_num: usize) -> Vec<(usize, usize)> {
    let mut byte_drops = Vec::new();
    let mut byte_count = 0;
    for line in reader.lines() {
        if byte_count >= max_byte_num {
            break;
        }
        let line = line.unwrap();
        let parts = line.split(",").collect_vec();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        byte_drops.push((x, y));
        byte_count += 1;
    }
    byte_drops
}

fn build_map(size: (usize, usize), byte_drops: Vec<(usize, usize)>) -> Vec<Vec<Field>> {
    let mut map = vec![vec![Field::Free; size.1]; size.0];
    for (time, (x, y)) in byte_drops.iter().enumerate() {
        map[*y][*x] = Field::Wall(time);
    }
    map[size.0 - 1][size.1 - 1] = Field::Exit;
    map
}

fn find_shortest_path(
    map: &Vec<Vec<Field>>,
    pos: (usize, usize),
    path: &HashSet<(usize, usize)>,
    shadow_map: &mut Vec<Vec<usize>>,
) -> Option<HashSet<(usize, usize)>> {
    if path.contains(&pos) {
        return None;
    }
    let mut path = path.clone();
    path.insert(pos);
    if map[pos.1][pos.0] == Field::Exit {
        return Some(path);
    }

    if shadow_map[pos.1][pos.0] <= path.len() {
        return None;
    } else {
        shadow_map[pos.1][pos.0] = path.len();
    }

    let mut returned_paths = Vec::with_capacity(4);
    for dir in NEIGHBORS.iter() {
        let new_pos = (
            pos.0 as isize + dir.as_coords().0,
            pos.1 as isize + dir.as_coords().1,
        );
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as isize
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as isize
        {
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        match map[new_pos.1][new_pos.0] {
            Field::Wall(_) => continue,
            _ => {}
        }
        if let Some(path) = find_shortest_path(map, new_pos, &path, shadow_map) {
            returned_paths.push(path);
        }
    }

    if returned_paths.is_empty() {
        None
    } else {
        Some(returned_paths.into_iter().min_by_key(|x| x.len()).unwrap())
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Field>>) {
    let mut output = String::new();
    for row in map.iter() {
        for field in row.iter() {
            output.push(match field {
                Field::Free => '.',
                Field::Wall(_) => '#',
                Field::Exit => 'E',
            });
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(
        reader: R,
        map_size: (usize, usize),
        num_byte_drop: usize,
    ) -> Result<usize> {
        let map = build_map(map_size, read_input(reader, num_byte_drop));
        // print_map(&map);
        let mut shadow_map = vec![vec![usize::MAX; map_size.1]; map_size.0];
        let path = find_shortest_path(&map, (0, 0), &HashSet::new(), &mut shadow_map).unwrap();
        Ok(path.len() - 1)
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), TEST_SIZE, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, REAL_SIZE, 1024)?);
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
