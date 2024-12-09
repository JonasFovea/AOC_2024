use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

#[derive(Debug, Clone)]
struct DiskFile {
    pub id: usize,
    pub start: usize,
    pub length: usize,
    pub next: Option<Box<DiskFile>>,
}

impl DiskFile {
    fn checksum(&self) -> usize {
        let mut sum = 0;
        let end = self.start + self.length;
        for i in self.start..end {
            sum += self.id * i;
        }
        sum + if let Some(next_file) = &self.next {
            next_file.checksum()
        } else {
            0
        }
    }

    fn is_end(&self) -> bool {
        self.next.is_none()
    }

    fn remaining_file_length(&self) -> usize {
        self.length
            + if let Some(nxt) = &self.next {
                nxt.remaining_file_length()
            } else {
                0
            }
    }

    fn is_in_file(&self, address: usize) -> bool {
        if self.length == 0 {
            return match &self.next {
                Some(nxt) => nxt.is_in_file(address),
                None => false,
            };
        }

        if self.start <= address && address < self.start + self.length {
            true
        } else {
            if let Some(nxt) = &self.next {
                nxt.is_in_file(address)
            } else {
                false
            }
        }
    }

    fn take_space(&mut self, space: &FreeSpace) -> (Option<FreeSpace>, bool) {
        if self.length == 0 {
            return (Some(space.clone()), true);
        }
        if space.start >= (self.start + self.length) {
            return (Some(space.clone()), true);
        }

        if self.length == space.length {
            self.start = space.start;
            (None, true)
        } else if self.length < space.length {
            self.start = space.start;
            let remaining_space = FreeSpace {
                start: self.start + self.length,
                length: space.length - self.length,
            };
            (Some(remaining_space), true)
        } else {
            let new_file_part = DiskFile {
                id: self.id,
                start: space.start,
                length: space.length,
                next: self.next.take(),
            };
            self.length -= space.length;
            self.next = Some(Box::new(new_file_part));
            (None, false)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct FreeSpace {
    pub start: usize,
    pub length: usize,
}

fn make_disk_map(input: String) -> (Vec<DiskFile>, Vec<FreeSpace>) {
    let mut input = input;
    if input.len() % 2 != 0 {
        input.push('0');
    }
    let mut files = Vec::with_capacity(input.len() / 2);
    let mut free_space = Vec::with_capacity(input.len() / 2);

    let mut address = 0;
    for (i, (c_f, c_s)) in input
        .chars()
        .step_by(2)
        .zip(input.chars().skip(1).step_by(2))
        .enumerate()
    {
        let file_len = usize::from_str_radix(format!("{c_f}").as_str(), 10).unwrap();
        let space_len = usize::from_str_radix(format!("{c_s}").as_str(), 10).unwrap();

        files.push(DiskFile {
            id: i,
            start: address,
            length: file_len,
            next: None,
        });
        address += file_len;

        if space_len != 0 {
            free_space.push(FreeSpace {
                length: space_len,
                start: address,
            });
            address += space_len;
        }
    }

    (files, free_space)
}

fn print_files(files: &Vec<DiskFile>, space: &Vec<FreeSpace>) {
    let total_file_length: usize = files.iter().map(|x| x.remaining_file_length()).sum();
    let total_space_length: usize = space.iter().map(|x1| x1.length).sum();
    let str_len = total_file_length + total_space_length;

    let mut out_vec = vec!['.'; str_len];
    for i in 0..str_len {
        'file_loop: for file in files {
            if file.is_in_file(i) {
                let file_id: Vec<char> = format!("{}", file.id).chars().collect();
                out_vec[i] = file_id[file_id.len() - 1];
                break 'file_loop;
            }
        }
        println!("{}", out_vec.iter().collect::<String>())
    }

    println!("{}", out_vec.iter().collect::<String>())
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().unwrap()?;
        let (mut files, mut space) = make_disk_map(line);

        space.reverse();
        let mut space_stack = space
            .iter()
            .filter(|x| x.length > 0)
            .cloned()
            .collect::<Vec<_>>();
        let mut files_rev = files.clone();
        files_rev.reverse();

        'file_loop: for file in &mut files_rev {
            if let Some(space) = space_stack.pop() {
                if space.length >= file.remaining_file_length() {
                    match file.take_space(&space) {
                        (None, _) => {}
                        (Some(remaining_space), _) => {
                            space_stack.push(remaining_space);
                        }
                    }
                } else {
                    let (remaining_space, _) = file.take_space(&space);
                    if let Some(returned_space) = remaining_space {
                        space_stack.push(returned_space);
                        continue 'file_loop;
                    }

                    'add_loop: while let Some(additional_space) = space_stack.pop() {
                        match file.take_space(&additional_space) {
                            (None, done) => {
                                if done {
                                    break 'add_loop;
                                }
                            }
                            (Some(remaining_space), _) => {
                                space_stack.push(remaining_space);
                                break 'add_loop;
                            }
                        }
                    }
                }
            } else {
                break 'file_loop;
            }
        }

        let mut sum = 0;
        for file in files_rev {
            sum += file.checksum();
        }
        Ok(sum)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
