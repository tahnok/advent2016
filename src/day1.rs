#[derive(PartialEq)]
#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[test]
fn test_turn_left() {
    assert_eq!(Direction::West, Direction::North.turn_left());
    assert_eq!(Direction::North, Direction::East.turn_left());
    assert_eq!(Direction::East, Direction::South.turn_left());
    assert_eq!(Direction::South, Direction::West.turn_left());
}

#[test]
fn test_turn_right() {
    assert_eq!(Direction::East, Direction::North.turn_right());
    assert_eq!(Direction::South, Direction::East.turn_right());
    assert_eq!(Direction::West, Direction::South.turn_right());
    assert_eq!(Direction::North, Direction::West.turn_right());
}

pub fn parse_input<'a>(input: &'a str) -> Vec<&'a str> {
    input.trim().split(", ").collect()
}

#[test]
fn parses_input_single_input() {
    let input = "L1";
    assert_eq!(vec!["L1"], parse_input(input));
}

#[test]
fn parses_input_multiple_input() {
    let input = "L1, R2";
    assert_eq!(vec!["L1", "R2"], parse_input(input));
}

#[test]
fn parses_input_newlines() {
    let input = "L1, R2\n";
    assert_eq!(vec!["L1", "R2"], parse_input(input));
}

pub fn calculate_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}
