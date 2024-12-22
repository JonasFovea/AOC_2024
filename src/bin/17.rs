use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const TEST_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

#[derive(Clone)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
    pc: usize,
    program: Vec<u128>,
    output: Vec<u128>,
}

impl Computer {
    fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let re_a = regex::Regex::new(r"Register A: (\d+)")?;
        let re_b = regex::Regex::new(r"Register B: (\d+)")?;
        let re_c = regex::Regex::new(r"Register C: (\d+)")?;
        let re_prog = regex::Regex::new(r"Program: ([\d,]+)")?;

        let input = reader.lines().map(|x| x.unwrap()).join(" ");

        let a = re_a
            .captures(&input)
            .ok_or(anyhow!("Register A not found"))?[1]
            .parse()?;
        let b = re_b
            .captures(&input)
            .ok_or(anyhow!("Register B not found"))?[1]
            .parse()?;
        let c = re_c
            .captures(&input)
            .ok_or(anyhow!("Register C not found"))?[1]
            .parse()?;
        let program = re_prog
            .captures(&input)
            .ok_or(anyhow!("Program not found"))?[1]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Self {
            a,
            b,
            c,
            pc: 0,
            program,
            output: Vec::new(),
        })
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "A: {}, B: {}, C: {}, PC: {}, \n\tProgramm:\t{:?}\n\tOutput:\t\t{:?}",
            self.a, self.b, self.c, self.pc, self.program, self.output
        );
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                0 => {
                    //adv
                    let operand = self.resolve_combo_operand(self.program[self.pc + 1]);
                    let numerator = self.a;
                    self.a = numerator / (1 << operand);
                }
                1 => {
                    //bxl
                    self.b = self.b ^ self.program[self.pc + 1];
                }
                2 => {
                    //bst
                    self.b = self.resolve_combo_operand(self.program[self.pc + 1]) % 8;
                }
                3 => {
                    //jnz
                    if self.a != 0 {
                        self.pc = self.program[self.pc + 1] as usize;
                        continue;
                    }
                }
                4 => {
                    //bxc
                    self.b = self.b ^ self.c;
                }
                5 => {
                    //out
                    self.output
                        .push(self.resolve_combo_operand(self.program[self.pc + 1]) % 8);
                }
                6 => {
                    //bdv
                    let operand = self.resolve_combo_operand(self.program[self.pc + 1]);
                    let numerator = self.a;
                    self.b = numerator / (1 << operand);
                }
                7 => {
                    //cdv
                    let operand = self.resolve_combo_operand(self.program[self.pc + 1]);
                    let numerator = self.a;
                    self.c = numerator / (1 << operand);
                }
                _ => {
                    panic!("Invalid opcode: {}", self.program[self.pc])
                }
            }
            self.pc += 2;
        }
    }

    fn resolve_combo_operand(&self, operand: u128) -> u128 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Unused operand detected: {}", operand),
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let mut computer = Computer::from_reader(reader)?;
        // computer.print();
        computer.run();
        Ok(computer.output.iter().join(","))
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST.as_bytes()))?
    );

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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        bail!("No start value for a found!")
    }

    assert_eq!(117440, part2(BufReader::new(TEST_2.as_bytes()))?);
    println!("Test passed");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
