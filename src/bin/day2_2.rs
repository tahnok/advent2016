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
    let mut position = '5';
    for directions in map.lines() {
        position = follow_directions(position, directions);
        code.push(position);
    }
    code.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}

pub fn follow_directions(start: char, directions: &str) -> char {
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

pub fn follow_direction(start: char, direction: char) -> char {
    match direction {
        'U' => move_up(start),
        'D' => move_down(start),
        'L' => move_left(start),
        'R' => move_right(start),
        _ => panic!("unknown direction: {}", direction)
    }
}

pub fn move_left(start: char) -> char {
    let mut left = HashMap::new();
    left.insert('1', '1');
    left.insert('2', '2');
    left.insert('3', '2');
    left.insert('4', '4');
    left.insert('5', '5');
    left.insert('6', '5');
    left.insert('7', '6');
    left.insert('8', '7');
    left.insert('9', '8');
    left.insert('A', 'A');
    left.insert('B', 'A');
    left.insert('C', 'B');
    left.insert('D', 'D');

    *left.get(&start).unwrap()
}

pub fn move_right(start: char) -> char {
    let mut right = HashMap::new();
    right.insert('1', '1');
    right.insert('2', '3');
    right.insert('3', '4');
    right.insert('4', '4');
    right.insert('5', '6');
    right.insert('6', '7');
    right.insert('7', '8');
    right.insert('8', '9');
    right.insert('9', '9');
    right.insert('A', 'B');
    right.insert('B', 'C');
    right.insert('C', 'C');
    right.insert('D', 'D');

    *right.get(&start).unwrap()
}


pub fn move_down(start: char) -> char {
    let mut down = HashMap::new();
    down.insert('1', '3');
    down.insert('2', '6');
    down.insert('3', '7');
    down.insert('4', '8');
    down.insert('5', '5');
    down.insert('6', 'A');
    down.insert('7', 'B');
    down.insert('8', 'C');
    down.insert('9', '9');
    down.insert('A', 'A');
    down.insert('B', 'D');
    down.insert('C', 'C');
    down.insert('D', 'D');

    *down.get(&start).unwrap()
}

pub fn move_up(start: char) -> char {
    let mut up = HashMap::new();
    up.insert('1', '1');
    up.insert('2', '2');
    up.insert('3', '1');
    up.insert('4', '4');
    up.insert('5', '5');
    up.insert('6', '2');
    up.insert('7', '3');
    up.insert('8', '4');
    up.insert('9', '9');
    up.insert('A', '6');
    up.insert('B', '7');
    up.insert('C', '8');
    up.insert('D', 'B');

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

        assert_eq!('5', follow_direction('5', direction));

        assert_eq!('9', follow_direction('9', direction));

        assert_eq!('1', follow_direction('1', direction));
    }

    #[test]
    fn it_follows_down_direction() {
        let direction = 'D';

        assert_eq!('A', follow_direction('6', direction));

        assert_eq!('9', follow_direction('9', direction));

        assert_eq!('6', follow_direction('2', direction));
    }

    #[test]
    fn it_follows_left_direction() {
        let direction = 'L';

        assert_eq!('5', follow_direction('5', direction));

        assert_eq!('1', follow_direction('1', direction));

        assert_eq!('8', follow_direction('9', direction));
    }

    #[test]
    fn it_follows_right_direction() {
        let direction = 'R';

        assert_eq!('6', follow_direction('5', direction));

        assert_eq!('4', follow_direction('3', direction));
    }

    #[test]
    fn it_follow_multiple_directions_in_square() {
        let directions = "URDL";

        assert_eq!('A', follow_directions('5', &directions));

        assert_eq!('2', follow_directions('3', &directions));
    }

    #[test]
    fn it_follows_directions_in_line() {
        let directions = "UU";

        assert_eq!('1', follow_directions('7', &directions));

        assert_eq!('1', follow_directions('1', &directions));
    }

    #[test]
    fn it_decodes_map() {
        let map = "ULL
        RRDDD
        LURDL
        UUUUD";

        assert_eq!("5DB3", decode_map(map));
    }

}
