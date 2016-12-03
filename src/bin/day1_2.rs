#[macro_use]
extern crate lazy_static;
extern crate regex;

#[path="../day1.rs"]
mod day1;

use day1::*;
use regex::Regex;
use std::io;
use std::ops::Range;

lazy_static! {
    static ref MOVEMENT_RE: Regex = Regex::new(r#"([RL])(\d+)"#).unwrap();
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match dist_to_first_revisit(&parse_input(&input)) {
                Ok(dist) => println!("{}", dist),
                Err(_) => println!("no revisits")
            }
        }
        Err(error) => println!("error: {}", error),
    }
}

fn dist_to_first_revisit(instructions: &[&str]) -> Result<i32, ()> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_direction = Direction::North;
    let mut visited = vec![(0, 0)];
    for instruction in instructions {
        let captures = MOVEMENT_RE.captures(instruction).unwrap();
        let direction = captures.at(1).unwrap();
        current_direction = match direction {
            "L" => current_direction.turn_left(),
            "R" => current_direction.turn_right(),
            _ => panic!("unknown direction"),
        };
        let distance = captures.at(2).unwrap().parse::<i32>().unwrap();
        let (new_x, new_y) = match current_direction {
            Direction::North => move_and_check(x, y, distance, 0),
            Direction::East => move_and_check(x, y, 0, distance),
            Direction::South => move_and_check(x, y, -distance, 0),
            Direction::West => move_and_check(x, y, 0, -distance),
        };

        match check_for_revisit(x, y, new_x, new_y, &mut visited) {
            Ok(d) => return Ok(d),
            Err(_) => (),
        }

        x = new_x;
        y = new_y;
        debug_hashset(instruction, &visited);
    }
    Err(())
}

fn debug_hashset(instruction: &str, visited: &Vec<(i32, i32)>) {
    println!("instruction: {}", instruction);
    for visit in visited {
        let (x, y) = *visit;
        println!("x: {}, y: {}", x, y);
    }
}

fn move_and_check(old_x: i32, old_y: i32, delta_x: i32, delta_y: i32) -> (i32, i32) {
    let new_x = old_x + delta_x;
    let new_y = old_y + delta_y;
    (new_x, new_y)
}

fn check_for_revisit(old_x: i32, old_y: i32, new_x: i32, new_y: i32, visited: &mut Vec<(i32, i32)>) -> Result<i32, ()> {
    println!("{}", steps(old_x, new_x));
    for mut step in old_x..new_x {
        step = step + 1;
        let pos = (step, old_y);
        println!("x: {}, y: {}", step, old_y);
        if visited.contains(&pos) {
            return Ok(calculate_distance_tuple(pos))
        } else {
            visited.push(pos);
        }
    }
    for mut step in old_y..new_y {
        step = step + 1;
        let pos = (new_x, step);
        println!("x: {}, y: {}", new_x, step);
        if visited.contains(&pos) {
            return Ok(calculate_distance_tuple(pos))
        } else {
            visited.push(pos);
        }
    }
    Err(())
}

fn steps(a: i32, b: i32) -> Vec<i32> {
    if b > a {
        Range{ start: a, end: b + 1}.collect()
    } else {
        Range{ end: a + 1, start: b }.rev().collect()
    }
}

#[test]
fn no_revisits() {
    let moves = ["R5", "L5", "R5", "R3"];
    assert!(dist_to_first_revisit(&moves).is_err())
}

#[test]
fn found_revisit() {
    let moves = ["R8", "R4", "R4", "R8"];
    assert_eq!(4, dist_to_first_revisit(&moves).unwrap());
}

#[test]
fn visits_all_steps_east() {
    let mut visited = vec![(0,0)];
    let _ = check_for_revisit(0, 0, 2, 0, &mut visited);
    assert_eq!(vec![(0,0), (1,0), (2,0)], visited);
}

#[test]
fn visits_all_steps_north() {
    let mut visited = vec![(0,0)];
    let _ = check_for_revisit(0, 0, 0, 2, &mut visited);
    assert_eq!(vec![(0,0), (0,1), (0,2)], visited);
}

#[test]
fn visits_all_steps_south() {
    let mut visited = vec![(0,0)];
    let _ = check_for_revisit(2, 0, 0, 0, &mut visited);
    assert_eq!(vec![(0,0), (0,1), (0,2)], visited);
}

#[test]
fn steps_works() {
    let expected = vec![1,2,3];
    let result: Vec<i32> = steps(1,3);
    assert_eq!(expected, result);
}

#[test]
fn steps_negative() {
    let expected = vec![-8,-7,-6];
    let result: Vec<i32> = steps(-8, -6);
    assert_eq!(expected, result);
}

#[test]
fn steps_reverse() {
    let expected = vec![3,2,1];
    let result: Vec<i32> = steps(3, 1);
    assert_eq!(expected, result);
}

#[test]
fn steps_reverse_negative() {
    let expected = vec![-6,-7,-8];
    let result: Vec<i32> = steps(-6, -8);
    assert_eq!(expected, result);
}
