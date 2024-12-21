use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

struct NumericKeypad {
    key_coords: HashMap<char, (usize, usize)>,
    current_key: char,
    allowed_coords: HashSet<(usize, usize)>,
}

impl NumericKeypad {
    fn new() -> Self {
        let mut key_coords = HashMap::new();
        key_coords.insert('A', (0, 0));
        key_coords.insert('0', (1, 0));
        key_coords.insert('1', (2, 1));
        key_coords.insert('2', (1, 1));
        key_coords.insert('3', (0, 1));
        key_coords.insert('4', (2, 2));
        key_coords.insert('5', (1, 2));
        key_coords.insert('6', (0, 2));
        key_coords.insert('7', (2, 3));
        key_coords.insert('8', (1, 3));
        key_coords.insert('9', (0, 3));

        let mut allowed_coords = HashSet::new();
        allowed_coords.extend(key_coords.values().cloned());
        NumericKeypad {
            key_coords,
            current_key: 'A',
            allowed_coords,
        }
    }

    fn move_to_key(&mut self, key: char) -> Vec<Move> {
        let mut moves_queue_1 = VecDeque::new();
        let mut moves_queue_2 = VecDeque::new();
        let (x, y) = self.key_coords[&self.current_key];
        let (x2, y2) = self.key_coords[&key];

        let y_moves = if y < y2 {
            std::iter::repeat(Move::Up).take(y2 - y)
        } else if y > y2 {
            std::iter::repeat(Move::Down).take(y - y2)
        } else {
            std::iter::repeat(Move::Right).take(0)
        };

        for m in y_moves {
            moves_queue_1.push_back(m);
            moves_queue_2.push_front(m);
        }

        let x_moves = if x < x2 {
            std::iter::repeat(Move::Left).take(x2 - x)
        } else if x > x2 {
            std::iter::repeat(Move::Right).take(x - x2)
        } else {
            std::iter::repeat(Move::Right).take(0)
        };

        for m in x_moves {
            moves_queue_1.push_front(m);
            moves_queue_2.push_back(m);
        }

        let mut moves_queue = moves_queue_1.clone();
        let mut moves = Vec::with_capacity(moves_queue.len());
        let mut current_pos = (x, y);
        while let Some(m) = moves_queue.pop_front() {
            let (dx, dy) = m.as_coords();
            let new_pos = (
                (current_pos.0 as isize + dx) as usize,
                (current_pos.1 as isize + dy) as usize,
            );
            if self.allowed_coords.contains(&new_pos) {
                moves.push(m);
                current_pos = new_pos;
            } else {
                moves_queue = moves_queue_2.clone();
                current_pos = (x, y);
                moves.clear();
            }
        }

        moves.push(Move::Activate);
        self.current_key = key;
        moves
    }
}

struct DirectionalKeypad {
    key_coords: HashMap<Move, (usize, usize)>,
    current_key: Move,
    allowed_coords: HashSet<(usize, usize)>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let mut key_coords = HashMap::new();
        key_coords.insert(Move::Activate, (0, 1));
        key_coords.insert(Move::Up, (1, 1));
        key_coords.insert(Move::Down, (1, 0));
        key_coords.insert(Move::Left, (2, 0));
        key_coords.insert(Move::Right, (0, 0));

        let mut allowed_coords = HashSet::new();
        allowed_coords.extend(key_coords.values().cloned());
        DirectionalKeypad {
            key_coords,
            current_key: Move::Activate,
            allowed_coords,
        }
    }

    fn move_to_key(&mut self, key: Move) -> Vec<Move> {
        let mut moves_queue_1 = VecDeque::new();
        let mut moves_queue_2 = VecDeque::new();
        let (x, y) = self.key_coords[&self.current_key];
        let (x2, y2) = self.key_coords[&key];

        let y_moves = if y < y2 {
            std::iter::repeat(Move::Up).take(y2 - y)
        } else if y > y2 {
            std::iter::repeat(Move::Down).take(y - y2)
        } else {
            std::iter::repeat(Move::Right).take(0)
        };

        for m in y_moves {
            moves_queue_1.push_back(m);
            moves_queue_2.push_front(m);
        }

        let x_moves = if x < x2 {
            std::iter::repeat(Move::Left).take(x2 - x)
        } else if x > x2 {
            std::iter::repeat(Move::Right).take(x - x2)
        } else {
            std::iter::repeat(Move::Right).take(0)
        };

        for m in x_moves {
            moves_queue_1.push_front(m);
            moves_queue_2.push_back(m);
        }

        let mut moves_queue = moves_queue_1.clone();
        let mut moves = Vec::with_capacity(moves_queue.len());
        let mut current_pos = (x, y);
        while let Some(m) = moves_queue.pop_front() {
            let (dx, dy) = m.as_coords();
            let new_pos = (
                (current_pos.0 as isize + dx) as usize,
                (current_pos.1 as isize + dy) as usize,
            );
            if self.allowed_coords.contains(&new_pos) {
                moves.push(m);
                current_pos = new_pos;
            } else {
                moves_queue = moves_queue_2.clone();
                current_pos = (x, y);
                moves.clear();
            }
        }

        moves.push(Move::Activate);
        self.current_key = key;
        moves
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Move {
    fn to_char(&self) -> char {
        match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
            Move::Activate => 'A',
        }
    }

    fn as_coords(&self) -> (isize, isize) {
        match self {
            Move::Up => (0, 1),
            Move::Down => (0, -1),
            Move::Left => (1, 0),
            Move::Right => (-1, 0),
            Move::Activate => (0, 0),
        }
    }
}

#[allow(dead_code)]
fn print_move_vec(moves: &[Move]) {
    let out = moves.iter().map(|m| m.to_char()).collect::<String>();
    println!("{}", out);
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum = 0;

        for line in reader.lines() {
            let line = line?;
            // println!("Making moves for passcode: {}", &line);
            let numeric_part: usize = line
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()?;
            let mut numeric_keypad = NumericKeypad::new();
            let mut numeric_moves = Vec::new();
            for c in line.chars() {
                numeric_moves.extend(numeric_keypad.move_to_key(c));
            }
            // print!("\t");
            // print_move_vec(&numeric_moves);

            let mut directional_keypad_1 = DirectionalKeypad::new();
            let mut directional_moves_1 = Vec::new();
            for m in numeric_moves {
                directional_moves_1.extend(directional_keypad_1.move_to_key(m));
            }
            // print!("\t");
            // print_move_vec(&directional_moves_1);

            let mut directional_keypad_2 = DirectionalKeypad::new();
            let mut directional_moves_2 = Vec::new();
            for m in directional_moves_1 {
                directional_moves_2.extend(directional_keypad_2.move_to_key(m));
            }
            // print!("\t");
            // print_move_vec(&directional_moves_2);
            println!("Complexity for {}: {:3} * {:3} = {:5}", line, directional_moves_2.len(), numeric_part, numeric_part * directional_moves_2.len());
            sum += numeric_part * directional_moves_2.len();
        }

        Ok(sum)
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);
    println!("Test passed");

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
