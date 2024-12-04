use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn count_xmas_in_field(field: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let num_rows = field.len();
    for row_idx in 0..num_rows {
        let num_cols = field[row_idx].len();
        for col_idx in 0..num_cols {
            match field[row_idx][col_idx] {
                'X' => {
                    // left to right
                    if col_idx + 3 < num_cols
                        && field[row_idx][col_idx + 1] == 'M'
                        && field[row_idx][col_idx + 2] == 'A'
                        && field[row_idx][col_idx + 3] == 'S'
                    {
                        count += 1;
                    }
                    // right to left
                    if col_idx > 2
                        && field[row_idx][col_idx - 1] == 'M'
                        && field[row_idx][col_idx - 2] == 'A'
                        && field[row_idx][col_idx - 3] == 'S'
                    {
                        count += 1;
                    }
                    // down
                    if row_idx + 3 < num_rows
                        && field[row_idx + 1][col_idx] == 'M'
                        && field[row_idx + 2][col_idx] == 'A'
                        && field[row_idx + 3][col_idx] == 'S'
                    {
                        count += 1;
                    }
                    // up
                    if row_idx > 2
                        && field[row_idx - 1][col_idx] == 'M'
                        && field[row_idx - 2][col_idx] == 'A'
                        && field[row_idx - 3][col_idx] == 'S'
                    {
                        count += 1;
                    }
                    // diagonal down right
                    if row_idx + 3 < num_rows
                        && col_idx + 3 < num_cols
                        && field[row_idx + 1][col_idx + 1] == 'M'
                        && field[row_idx + 2][col_idx + 2] == 'A'
                        && field[row_idx + 3][col_idx + 3] == 'S'
                    {
                        count += 1;
                    }
                    // diagonal up right
                    if row_idx > 2
                        && col_idx + 3 < num_cols
                        && field[row_idx - 1][col_idx + 1] == 'M'
                        && field[row_idx - 2][col_idx + 2] == 'A'
                        && field[row_idx - 3][col_idx + 3] == 'S'
                    {
                        count += 1;
                    }
                    // diagonal down left
                    if row_idx + 3 < num_rows
                        && col_idx > 2
                        && field[row_idx + 1][col_idx - 1] == 'M'
                        && field[row_idx + 2][col_idx - 2] == 'A'
                        && field[row_idx + 3][col_idx - 3] == 'S'
                    {
                        count += 1;
                    }
                    // diagonal up left
                    if row_idx > 2
                        && col_idx > 2
                        && field[row_idx - 1][col_idx - 1] == 'M'
                        && field[row_idx - 2][col_idx - 2] == 'A'
                        && field[row_idx - 3][col_idx - 3] == 'S'
                    {
                        count += 1;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    count
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let char_field: Vec<Vec<char>> = reader
            .lines()
            .map(|x| x.unwrap().chars().collect())
            .collect();
        let answer = count_xmas_in_field(&char_field);
        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
