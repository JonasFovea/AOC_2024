use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn read_char_field<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
    let mut field = Vec::new();
    for line in reader.lines() {
        let line = line?;
        field.push(line.chars().collect());
    }
    Ok(field)
}

fn make_frequency_map(field: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    for (y, row) in field.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                let entry = map.entry(c).or_insert(Vec::new());
                entry.push((y, x));
            }
        }
    }
    map
}

fn get_antinodes_for_frequency(
    antenna_positions: &Vec<(usize, usize)>,
    field_size: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    // check all pairs of antenna positions and if possible add antinodes

    for i in 0..antenna_positions.len() - 1 {
        for j in i + 1..antenna_positions.len() {
            let (x1, y1) = antenna_positions[i];
            let (x2, y2) = antenna_positions[j];

            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            let gcd = greatest_common_divisor(dx.abs() as usize, dy.abs() as usize);
            let dx_step = dx / gcd as isize;
            let dy_step = dy / gcd as isize;

            let mut step = 0;
            while is_step_in_field(x1, y1, dx_step * step, dy_step * step, field_size) {
                let (x, y) = n_step(dx_step, dy_step, step as usize);
                let x = x1 as isize + x;
                let y = y1 as isize + y;
                if dist_is_two_to_one((x1, y1), (x2, y2), (x as usize, y as usize)) {
                    antinodes.push((x as usize, y as usize));
                }
                step += 1;
            }
            // also check negative direction
            step = 1;
            while is_step_in_field(x1, y1, -dx_step * step, -dy_step * step, field_size) {
                let (x, y) = n_step(-dx_step, -dy_step, step as usize);
                let x = x1 as isize + x;
                let y = y1 as isize + y;
                if dist_is_two_to_one((x1, y1), (x2, y2), (x as usize, y as usize)) {
                    antinodes.push((x as usize, y as usize));
                }
                step += 1;
            }

        }
    }

    antinodes
}

fn n_step(dx: isize, dy: isize, steps: usize) -> (isize, isize) {
    (dx * steps as isize, dy * steps as isize)
}

fn is_step_in_field(x: usize, y: usize, dx: isize, dy: isize, field_size: (usize, usize)) -> bool {
    let (width, height) = field_size;
    let new_x = x as isize + dx;
    let new_y = y as isize + dy;
    new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize
}

fn dist_is_two_to_one(pos_1: (usize, usize), pos_2: (usize, usize), point: (usize, usize)) -> bool {
    let dx1 = pos_1.0 as isize - point.0 as isize;
    let dy1 = pos_1.1 as isize - point.1 as isize;
    let dx2 = pos_2.0 as isize - point.0 as isize;
    let dy2 = pos_2.1 as isize - point.1 as isize;

    if dx1 == 0 && dx2 == 0 {
        return (dy1 / dy2 == 2) || (dy2 / dy1 == 2);
    } else if (dx1 == 0 && dx2 != 0) || (dx1 != 0 && dx2 == 0) {
        return false;
    }

    if dy1 == 0 && dy2 == 0 {
        return (dx1 / dx2 == 2) || (dx2 / dx1 == 2);
    } else if (dy1 == 0 && dy2 != 0) || (dy1 != 0 && dy2 == 0) {
        return false;
    }

    (dx1 / dx2 == 2 && dy1 / dy2 == 2) || (dx2 / dx1 == 2 && dy2 / dy1 == 2)
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let field = read_char_field(reader)?;
        let frequency_map = make_frequency_map(&field);
        // println!("Antennas positioned by frequency: {:?}", frequency_map);
        let mut antinodes: Vec<Vec<(usize, usize)>> = Vec::new();
        for (_, positions) in frequency_map.iter() {
            antinodes.push(get_antinodes_for_frequency(
                positions,
                (field.len(), field[0].len()),
            ));
        }

        let mut unique_antinodes: HashSet<_> = HashSet::new();
        for antinode in antinodes.iter() {
            for pos in antinode.iter() {
                unique_antinodes.insert(*pos);
            }
        }

        let answer = unique_antinodes.len();
        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

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
