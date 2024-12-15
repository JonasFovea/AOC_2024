use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const TEST_2: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Robot,
    Box(usize, usize),
}

impl Cell {
    fn from_char(c: char, x: usize, y: usize) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            '@' => Cell::Robot,
            'O' => Cell::Box(x, y),
            _ => panic!("Invalid cell type: {}", c),
        }
    }

    fn gps_coordinate(&self) -> Option<usize> {
        match self {
            Cell::Box(x, y) => Some(100 * y + x),
            _ => None,
        }
    }
}

struct Field {
    size: (usize, usize),
    cells: Vec<Vec<Cell>>,
    robot_position: (usize, usize),
}

impl Field {
    fn from_str(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut robot_position = None;
        let mut size = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let cell = Cell::from_char(c, x, y);
                match cell {
                    Cell::Robot => robot_position = Some((x, y)),
                    _ => {}
                }
                row.push(cell);
            }
            size.1 = y + 1;
            size.0 = line.len();
            cells.push(row);
        }
        Field {
            size,
            cells,
            robot_position: robot_position.unwrap(),
        }
    }

    fn enlarge(&mut self) {
        let new_size = (self.size.0 * 2, self.size.1);

        let mut new_cells = vec![vec![Cell::Empty; new_size.0]; new_size.1];
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Wall => {
                        new_cells[y][x * 2] = Cell::Wall;
                        new_cells[y][x * 2 + 1] = Cell::Wall;
                    }
                    Cell::Box(_, _) => {
                        new_cells[y][x * 2] = Cell::Box(x * 2, y);
                    }
                    Cell::Robot => {
                        new_cells[y][x * 2] = Cell::Robot;
                    }
                    _ => {}
                }
            }
        }

        self.cells = new_cells;
        self.size = new_size;
        self.robot_position = (self.robot_position.0 * 2, self.robot_position.1);
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut out = String::new();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match &self.cells[y][x] {
                    Cell::Wall => out.push('#'),
                    Cell::Empty => out.push('.'),
                    Cell::Robot => out.push('@'),
                    Cell::Box(_, _) => out.push('O'),
                }
            }
            out.push('\n');
        }
        println!("{}", out);
    }

    #[allow(dead_code)]
    fn print_wide(&self) {
        let mut out = Vec::new();
        for y in 0..self.size.1 {
            let mut line = Vec::new();
            for x in 0..self.size.0 {
                match &self.cells[y][x] {
                    Cell::Wall => line.push('#'),
                    Cell::Empty => line.push('.'),
                    Cell::Robot => line.push('@'),
                    Cell::Box(_, _) => line.push('O'),
                }
            }
            line.push('\n');
            out.push(line);
        }

        for cell in self.cells.iter().flatten() {
            match cell {
                Cell::Box(x, y) => {
                    out[*y][*x] = '[';
                    out[*y][*x + 1] = ']';
                }
                _ => {}
            }
        }

        println!("{}", out.iter().flatten().collect::<String>());
    }

    fn robot_push(&mut self, direction: Direction) {
        let (dx, dy) = direction.as_coords();
        let (x, y) = self.robot_position;
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if !self.is_valid_position(nx, ny) {
            return;
        }

        match self.cells[ny as usize][nx as usize] {
            Cell::Wall => return,
            Cell::Empty => {
                self.robot_position = (nx as usize, ny as usize);
                self.cells[y][x] = Cell::Empty;
                self.cells[self.robot_position.1][self.robot_position.0] = Cell::Robot;
                return;
            }
            Cell::Box(_, _) => {
                if self.push((nx as usize, ny as usize), direction) {
                    self.robot_position = (nx as usize, ny as usize);
                    self.cells[y][x] = Cell::Empty;
                    self.cells[self.robot_position.1][self.robot_position.0] = Cell::Robot;
                }
            }
            _ => {}
        }
    }

    fn robot_push_wide(&mut self, direction: Direction) {
        let (x, y) = self.robot_position;
        let (dx, dy) = direction.as_coords();
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if !self.is_valid_position(nx, ny) {
            return;
        }

        let (nx, ny) = (nx as usize, ny as usize);
        let (lnx, lny) = (nx - 1, ny);

        match direction {
            Direction::Left => match (self.cells[lny][lnx], self.cells[ny][nx]) {
                (Cell::Box(_, _), Cell::Empty) => {
                    if self.push_wide((lnx, lny), direction, true) {
                        self.robot_position = (nx, ny);
                        self.cells[ny][nx] = Cell::Robot;
                        self.cells[y][x] = Cell::Empty;
                    }
                }
                (_, Cell::Empty) => {
                    self.robot_position = (nx, ny);
                    self.cells[ny][nx] = Cell::Robot;
                    self.cells[y][x] = Cell::Empty;
                }
                _ => {
                    return;
                }
            },
            Direction::Right => match self.cells[ny][nx] {
                Cell::Empty => {
                    self.robot_position = (nx, ny);
                    self.cells[ny][nx] = Cell::Robot;
                    self.cells[y][x] = Cell::Empty;
                }
                Cell::Box(_, _) => {
                    if self.push_wide((nx, ny), direction, true) {
                        self.robot_position = (nx, ny);
                        self.cells[ny][nx] = Cell::Robot;
                        self.cells[y][x] = Cell::Empty;
                    }
                }
                _ => {
                    return;
                }
            },
            Direction::Up | Direction::Down => match (self.cells[lny][lnx], self.cells[ny][nx]) {
                (_, Cell::Box(_, _)) => {
                    if self.push_wide((nx, ny), direction, true) {
                        self.robot_position = (nx, ny);
                        self.cells[ny][nx] = Cell::Robot;
                        self.cells[y][x] = Cell::Empty;
                    }
                }
                (Cell::Box(_, _), Cell::Empty) => {
                    if self.push_wide((lnx, lny), direction, true) {
                        self.robot_position = (nx, ny);
                        self.cells[ny][nx] = Cell::Robot;
                        self.cells[y][x] = Cell::Empty;
                    }
                }
                (_, Cell::Empty) => {
                    self.robot_position = (nx, ny);
                    self.cells[ny][nx] = Cell::Robot;
                    self.cells[y][x] = Cell::Empty;
                }
                _ => {
                    return;
                }
            },
        }
    }

    fn push(&mut self, pos: (usize, usize), direction: Direction) -> bool {
        let (dx, dy) = direction.as_coords();
        let (x, y) = pos;
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if !self.is_valid_position(nx, ny) {
            return false;
        }

        let (nx, ny) = (nx as usize, ny as usize);

        match self.cells[ny][nx] {
            Cell::Empty => {
                self.cells[ny][nx] = Cell::Box(nx, ny);
                true
            }
            Cell::Box(_, _) => {
                if self.push((nx, ny), direction) {
                    self.cells[ny][nx] = Cell::Box(nx, ny);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn push_wide(&mut self, pos: (usize, usize), direction: Direction, execute: bool) -> bool {
        let (x, y) = pos;

        match self.cells[y][x] {
            Cell::Empty => true,
            Cell::Wall => false,
            Cell::Box(_, _) => match direction {
                Direction::Left => {
                    let (nx, ny) = (x - 1, y);
                    let (lnx, lny) = (nx - 1, ny);
                    match (self.cells[lny][lnx], self.cells[ny][nx]) {
                        (Cell::Box(_, _), Cell::Empty) => {
                            if self.push_wide((lnx, lny), direction, execute) {
                                if execute {
                                    self.cells[ny][nx] = Cell::Box(nx, ny);
                                    self.cells[y][x] = Cell::Empty;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        (_, Cell::Empty) => {
                            if execute {
                                self.cells[ny][nx] = Cell::Box(nx, ny);
                                self.cells[y][x] = Cell::Empty;
                            }
                            true
                        }
                        _ => false,
                    }
                }
                Direction::Right => {
                    let (nx, ny) = (x + 1, y);
                    if self.push_wide((nx + 1, ny), direction, execute) {
                        if execute {
                            self.cells[ny][nx] = Cell::Box(nx, ny);
                            self.cells[y][x] = Cell::Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                Direction::Up => {
                    let (nx, ny) = (x, y - 1);
                    let (lnx, lny) = (x - 1, y - 1);
                    let (rnx, rny) = (x + 1, y - 1);

                    let ((bx, by), (ex, ey)) = match (
                        self.cells[lny][lnx],
                        self.cells[ny][nx],
                        self.cells[rny][rnx],
                    ) {
                        (Cell::Box(_, _), Cell::Empty, Cell::Box(_, _)) => ((lnx, lny), (rnx, rny)),
                        (Cell::Box(_, _), Cell::Empty, Cell::Empty) => ((lnx, lny), (nx, ny)),
                        _ => ((nx, ny), (rnx, rny)),
                    };

                    let test_box_place = self.push_wide((bx, by), direction, false);
                    let test_empty_place = self.push_wide((ex, ey), direction, false);
                    if !(test_box_place && test_empty_place) {
                        return false;
                    }
                    self.push_wide((bx, by), direction, execute);
                    self.push_wide((ex, ey), direction, execute);

                    if execute {
                        self.cells[ny][nx] = Cell::Box(nx, ny);
                        self.cells[rny][rnx] = Cell::Empty;
                        self.cells[y][x] = Cell::Empty;
                    }

                    true
                }
                Direction::Down => {
                    let (nx, ny) = (x, y + 1);
                    let (lnx, lny) = (x - 1, y + 1);
                    let (rnx, rny) = (x + 1, y + 1);

                    let ((bx, by), (ex, ey)) = match (
                        self.cells[lny][lnx],
                        self.cells[ny][nx],
                        self.cells[rny][rnx],
                    ) {
                        (Cell::Box(_, _), Cell::Empty, Cell::Box(_, _)) => ((lnx, lny), (rnx, rny)),
                        (Cell::Box(_, _), Cell::Empty, Cell::Empty) => ((lnx, lny), (nx, ny)),
                        _ => ((nx, ny), (rnx, rny)),
                    };

                    let test_box_place = self.push_wide((bx, by), direction, false);
                    let test_empty_place = self.push_wide((ex, ey), direction, false);
                    if !(test_box_place && test_empty_place) {
                        return false;
                    }
                    self.push_wide((bx, by), direction, execute);
                    self.push_wide((ex, ey), direction, execute);

                    if execute {
                        self.cells[ny][nx] = Cell::Box(nx, ny);
                        self.cells[rny][rnx] = Cell::Empty;
                        self.cells[y][x] = Cell::Empty;
                    }

                    true
                }
            },
            _ => panic!("Robot should not be pushed!"),
        }
    }

    fn is_valid_position(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.size.0 as isize && y < self.size.1 as isize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }

    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn as_coords(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn parse_path(input: &str) -> Vec<Direction> {
        input
            .lines()
            .join("")
            .trim_end()
            .chars()
            .map(Direction::from_char)
            .collect()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let input_split = input.split("\n\n").collect::<Vec<&str>>();

        let mut field = Field::from_str(input_split[0]);
        let path = Direction::parse_path(input_split[1]);

        for direction in path {
            field.robot_push(direction);
        }

        let coordinate_sum = field
            .cells
            .iter()
            .flatten()
            .filter_map(Cell::gps_coordinate)
            .sum::<usize>();

        Ok(coordinate_sum)
    }

    assert_eq!(2028, part1(BufReader::new(TEST_2.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST_1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let input_split = input.split("\n\n").collect::<Vec<&str>>();

        let mut field = Field::from_str(input_split[0]);
        let path = Direction::parse_path(input_split[1]);

        field.enlarge();

        for direction in &path {
            field.robot_push_wide(*direction);
        }

        let coordinate_sum = field
            .cells
            .iter()
            .flatten()
            .filter_map(Cell::gps_coordinate)
            .sum::<usize>();

        Ok(coordinate_sum)
    }

    assert_eq!(9021, part2(BufReader::new(TEST_1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
