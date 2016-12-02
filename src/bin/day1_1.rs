#[macro_use]
extern crate lazy_static;
extern crate regex;

#[path="../day1.rs"]
mod day1;

use day1::*;
use regex::Regex;
use std::io;

lazy_static! {
    static ref MOVEMENT_RE: Regex = Regex::new(r#"([RL])(\d+)"#).unwrap();
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let dist = follow_instructions(&parse_input(&input));
            println!("{}", dist);
        }
        Err(error) => println!("error: {}", error),
    }
}

fn follow_instructions(instructions: &[&str]) -> i32 {
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
    calculate_distance(x_position, y_position)
}

#[test]
fn follow_instructions_one_move_left() {
    let left = ["L1"];
    assert_eq!(1, follow_instructions(&left));
}

#[test]
fn follow_instructions_two_moves_left() {
    let left = ["L2"];
    assert_eq!(2, follow_instructions(&left));
}

#[test]
fn follow_instructions_one_move_right() {
    let right = ["R1"];
    assert_eq!(1, follow_instructions(&right));
}

#[test]
fn follow_instructions_two_moves_right() {
    let right = ["R2"];
    assert_eq!(2, follow_instructions(&right));
}

#[test]
fn follow_instructions_right_then_left() {
    let moves = ["R2", "L3"];
    assert_eq!(5, follow_instructions(&moves));
}

#[test]
fn follow_instructions_all_rights() {
    let moves = ["R2", "R2", "R2"];
    assert_eq!(2, follow_instructions(&moves));
}

#[test]
fn follow_instructions_complex() {
    let moves = ["R5", "L5", "R5", "R3"];
    assert_eq!(12, follow_instructions(&moves));
}
