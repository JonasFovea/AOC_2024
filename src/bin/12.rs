use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const TEST2: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const TEST3: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    fields: HashSet<(usize, usize)>,
    max_row: usize,
    min_row: usize,
    max_col: usize,
    min_col: usize,
    field_size: (usize, usize),
}

impl Region {
    fn from_pos(plant: char, pos: (usize, usize), field_size: (usize, usize)) -> Self {
        Region {
            plant,
            fields: vec![pos].into_iter().collect(),
            max_row: pos.0,
            min_row: pos.0,
            max_col: pos.1,
            min_col: pos.1,
            field_size,
        }
    }

    fn price(&self) -> usize {
        self.perimeter_count() * self.fields.len()
    }

    fn perimeter_count(&self) -> usize {
        let mut fences = Vec::new();
        let mut count = 0;
        for field in &self.fields {
            if field.0 > 0 {
                if !self.is_inside((field.0 - 1, field.1)) {
                    fences.push((field.0 as isize - 1, field.1 as isize));
                    count += 1;
                }
            } else {
                fences.push((field.0 as isize - 1, field.1 as isize));
                count += 1;
            }

            if !self.is_inside((field.0 + 1, field.1)) {
                fences.push((field.0 as isize + 1, field.1 as isize));
                count += 1;
            }
            if field.1 > 0 {
                if !self.is_inside((field.0, field.1 - 1)) {
                    fences.push((field.0 as isize, field.1 as isize - 1));
                    count += 1;
                }
            } else {
                fences.push((field.0 as isize, field.1 as isize - 1));
                count += 1;
            }

            if !self.is_inside((field.0, field.1 + 1)) {
                fences.push((field.0 as isize, field.1 as isize + 1));
                count += 1;
            }
        }

        count
    }

    fn is_inside(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        if row < self.min_row || row > self.max_row || col < self.min_col || col > self.max_col {
            return false;
        }
        self.fields.contains(&pos)
    }

    fn append(&mut self, pos: (usize, usize)) -> bool {
        for field in &self.fields {
            let diff_row = pos.0 as isize - field.0 as isize;
            let diff_col = pos.1 as isize - field.1 as isize;

            if diff_row.abs() + diff_col.abs() < 2 {
                self.fields.insert(pos);
                if pos.0 < self.min_row {
                    self.min_row = pos.0;
                }
                if pos.0 > self.max_row {
                    self.max_row = pos.0;
                }
                if pos.1 < self.min_col {
                    self.min_col = pos.1;
                }
                if pos.1 > self.max_col {
                    self.max_col = pos.1;
                }

                return true;
            }
        }

        false
    }

    fn print_in_field(&self, field_size: (usize, usize)) {
        let mut char_field = vec![vec!['.'; field_size.1]; field_size.0];
        for pos in &self.fields {
            char_field[pos.0][pos.1] = self.plant;
        }
        println!(
            "Region of {}x{}, needs {} fences ->price: {}\n{}",
            self.fields.len(),
            self.plant,
            self.perimeter_count(),
            self.price(),
            char_field
                .iter()
                .map(|x| { x.iter().collect::<String>() })
                .join("\n")
        );
    }

    fn can_be_merged(&self, other: &Region) -> bool {
        if self.plant != other.plant {
            return false;
        }

        for field in &self.fields {
            if field.0 > 1 && other.is_inside((field.0 - 1, field.1)) {
                return true;
            } else if field.0 < self.field_size.0 - 1 && other.is_inside((field.0 + 1, field.1)) {
                return true;
            } else if field.1 > 0 && other.is_inside((field.0, field.1 - 1)) {
                return true;
            } else if field.1 < self.field_size.1 - 1 && other.is_inside((field.0, field.1 + 1)) {
                return true;
            }
        }

        false
    }

    fn merge(&mut self, other: Region) {
        self.fields.extend(other.fields);
        self.max_row = self.max_row.max(other.max_row);
        self.min_row = self.min_row.min(other.min_row);
        self.max_col = self.max_col.max(other.max_col);
        self.min_col = self.min_col.min(other.min_col);
    }
}

fn build_regions(field: &Vec<Vec<char>>) -> Vec<Region> {
    let field_size = (field.len(), field[0].len());

    let mut plant_pos = HashMap::new();
    for (r_idx, row) in field.iter().enumerate() {
        for (c_idx, plant) in row.iter().enumerate() {
            let pos = (r_idx, c_idx);
            match plant_pos.get_mut(plant) {
                None => {
                    plant_pos.insert(*plant, vec![pos]);
                }
                Some(list) => {
                    list.push(pos);
                }
            }
        }
    }

    let mut regions: Vec<Region> = Vec::new();
    for (plant, positions) in plant_pos {
        let mut plant_regions = Vec::new();
        'pos_loop: for pos in positions {
            if plant_regions.len() == 0 {
                plant_regions.push(Region::from_pos(plant, pos, field_size));
                continue;
            }
            for reg in &mut plant_regions {
                if reg.append(pos) {
                    continue 'pos_loop;
                }
            }
            plant_regions.push(Region::from_pos(plant, pos, field_size));
        }

        'merge_loop: loop {
            if plant_regions.len() < 2 {
                break 'merge_loop;
            }

            let mut i = 0;
            while i < plant_regions.len() - 1 {
                let mut j = i + 1;
                while j < plant_regions.len() {
                    if plant_regions[i].can_be_merged(&plant_regions[j]) {
                        let other = plant_regions.remove(j);
                        // println!("Merging\n\t{:?}\nwith\n\t{:?}\n", plant_regions[i], other);
                        plant_regions[i].merge(other);
                        continue 'merge_loop;
                    }
                    j += 1;
                }
                i += 1;
            }
            break 'merge_loop;
        }
        regions.extend(plant_regions);
    }

    regions
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let field: Vec<Vec<char>> = reader
            .lines()
            .map(|x| x.unwrap().chars().collect())
            .collect();
        let regions = build_regions(&field);

        // println!("\n");
        // let field_rows = field.len();
        // let field_cols = field[0].len();
        // for reg in &regions {
        //     println!("{:?}", reg);
        //     reg.print_in_field((field_rows, field_cols));
        //     println!();
        // }

        let answer = regions.iter().map(|r| r.price()).sum();
        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(140, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST3.as_bytes()))?);

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
