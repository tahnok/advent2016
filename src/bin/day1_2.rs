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
    for mut step in old_x..new_x {
        step = step + 1;
        let pos = (step, old_y);
        // println!("x: {}, y: {}", step, old_y);
        if visited.contains(&pos) {
            return Ok(calculate_distance_tuple(pos))
        } else {
            visited.push(pos);
        }
    }
    for mut step in old_y..new_y {
        step = step + 1;
        let pos = (new_x, step);
        // println!("x: {}, y: {}", new_x, step);
        if visited.contains(&pos) {
            return Ok(calculate_distance_tuple(pos))
        } else {
            visited.push(pos);
        }
    }
    Err(())
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
