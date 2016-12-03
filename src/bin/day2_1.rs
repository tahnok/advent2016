use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let code = decode_map(&input);
    println!("{}", code);
}

pub fn decode_map(map: &str) -> String {
    let mut code = vec![];
    let mut position = 5;
    for directions in map.lines() {
        position = follow_directions(position, directions);
        code.push(position);
    }
    code.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}

pub fn follow_directions(start: i32, directions: &str) -> i32 {
    let mut current = start;
    let parsed_directions = parse_directions(directions);
    for direction in parsed_directions {
        current = follow_direction(current, direction);
    }
    current
}

pub fn parse_directions(directions: &str) -> Vec<char> {
    directions.trim().chars()
        .collect::<Vec<char>>()
}

pub fn follow_direction(start: i32, direction: char) -> i32 {
    match direction {
        'U' => move_up(start),
        'D' => move_down(start),
        'L' => move_left(start),
        'R' => move_right(start),
        _ => panic!("unknown direction: {}", direction)
    }
}

pub fn move_left(start: i32) -> i32 {
    let mut left = HashMap::new();
    left.insert(1, 1);
    left.insert(2, 1);
    left.insert(3, 2);
    left.insert(4, 4);
    left.insert(5, 4);
    left.insert(6, 5);
    left.insert(7, 7);
    left.insert(8, 7);
    left.insert(9, 8);

    *left.get(&start).unwrap()
}

pub fn move_right(start: i32) -> i32 {
    let mut right = HashMap::new();
    right.insert(1, 2);
    right.insert(2, 3);
    right.insert(3, 3);
    right.insert(4, 5);
    right.insert(5, 6);
    right.insert(6, 6);
    right.insert(7, 8);
    right.insert(8, 9);
    right.insert(9, 9);

    *right.get(&start).unwrap()
}


pub fn move_down(start: i32) -> i32 {
    let mut down = HashMap::new();
    down.insert(1, 4);
    down.insert(2, 5);
    down.insert(3, 6);
    down.insert(4, 7);
    down.insert(5, 8);
    down.insert(6, 9);
    down.insert(7, 7);
    down.insert(8, 8);
    down.insert(9, 9);

    *down.get(&start).unwrap()
}

pub fn move_up(start: i32) -> i32 {
    let mut up = HashMap::new();
    up.insert(1, 1);
    up.insert(2, 2);
    up.insert(3, 3);
    up.insert(4, 1);
    up.insert(5, 2);
    up.insert(6, 3);
    up.insert(7, 4);
    up.insert(8, 5);
    up.insert(9, 6);

    *up.get(&start).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_parses_directions() {
        let input = "ULL";
        let output = vec!['U', 'L', 'L'];

        assert_eq!(output, parse_directions(input));
    }

    #[test]
    fn it_follows_up_direction() {
        let direction = 'U';

        assert_eq!(2, follow_direction(5, direction));

        assert_eq!(6, follow_direction(9, direction));

        assert_eq!(1, follow_direction(1, direction));
    }

    #[test]
    fn it_follows_down_direction() {
        let direction = 'D';

        assert_eq!(9, follow_direction(6, direction));

        assert_eq!(9, follow_direction(9, direction));

        assert_eq!(5, follow_direction(2, direction));
    }

    #[test]
    fn it_follows_left_direction() {
        let direction = 'L';

        assert_eq!(4, follow_direction(5, direction));

        assert_eq!(1, follow_direction(1, direction));

        assert_eq!(8, follow_direction(9, direction));
    }

    #[test]
    fn it_follows_right_direction() {
        let direction = 'R';

        assert_eq!(6, follow_direction(5, direction));

        assert_eq!(3, follow_direction(3, direction));
    }

    #[test]
    fn it_follow_multiple_directions_in_square() {
        let directions = "URDL";

        assert_eq!(5, follow_directions(5, &directions));

        assert_eq!(5, follow_directions(3, &directions));
    }

    #[test]
    fn it_follows_directions_in_line() {
        let directions = "UU";

        assert_eq!(1, follow_directions(7, &directions));

        assert_eq!(1, follow_directions(1, &directions));
    }

    #[test]
    fn it_decodes_map() {
        let map = "ULL
        RRDDD
        LURDL
        UUUUD";

        assert_eq!("1985", decode_map(map));
    }

}
