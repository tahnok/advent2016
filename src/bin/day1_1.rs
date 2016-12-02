#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;

lazy_static! {
    static ref MOVEMENT_RE: Regex = Regex::new(r#"([RL])(\d+)"#).unwrap();
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let dist = calculate_distance(&parse_input(&input));
            println!("{}", dist);
        }
        Err(error) => println!("error: {}", error),
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

fn calculate_distance(instructions: &[&str]) -> i32 {
    let mut x_position: i32 = 0;
    let mut y_position: i32 = 0;
    let mut current_direction = Direction::North;
    for instruction in instructions {
        let captures = MOVEMENT_RE.captures(instruction).unwrap();
        let direction = captures.at(1).unwrap();
        current_direction = match direction {
            "L" => current_direction.turn_left(),
            "R" => current_direction.turn_right(),
            _ => panic!("unknown direction"),
        };
        let distance = captures.at(2).unwrap().parse::<i32>().unwrap();
        match current_direction {
            Direction::North => x_position = x_position + distance,
            Direction::East => y_position = y_position + distance,
            Direction::South => x_position = x_position - distance,
            Direction::West => y_position = y_position - distance,
        }
        println!("x: {}, y: {}", x_position, y_position);
    }
    x_position.abs() + y_position.abs()
}

fn parse_input<'a>(input: &'a str) -> Vec<&'a str> {
    input.trim().split(", ").collect()
}

#[test]
fn calculate_distance_one_move_left() {
    let left = ["L1"];
    assert_eq!(1, calculate_distance(&left));
}

#[test]
fn calculate_distance_two_moves_left() {
    let left = ["L2"];
    assert_eq!(2, calculate_distance(&left));
}

#[test]
fn calculate_distance_one_move_right() {
    let right = ["R1"];
    assert_eq!(1, calculate_distance(&right));
}

#[test]
fn calculate_distance_two_moves_right() {
    let right = ["R2"];
    assert_eq!(2, calculate_distance(&right));
}

#[test]
fn calculate_distance_right_then_left() {
    let moves = ["R2", "L3"];
    assert_eq!(5, calculate_distance(&moves));
}

#[test]
fn calculate_distance_all_rights() {
    let moves = ["R2", "R2", "R2"];
    assert_eq!(2, calculate_distance(&moves));
}

#[test]
fn calculate_distance_complex() {
    let moves = ["R5", "L5", "R5", "R3"];
    assert_eq!(12, calculate_distance(&moves));
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
