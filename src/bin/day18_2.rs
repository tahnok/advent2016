use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let start = parse(&input.trim());
    let board = rows(start, 400000);
    let safe = count_safe(&board);
    println!("{}", safe);
}

pub fn parse(input: &str) -> Vec<bool> {
    let mut output = Vec::with_capacity(input.len());
    for x in input.chars()  {
        match x {
            '.' => output.push(false),
            '^' => output.push(true),
            _ => panic!("unknown char"),
        }
    }
    output
}

pub fn unparse(input: &[bool]) -> String {
    let mut output = String::new();
    for &x in input {
        if x {
            output.push('^');
        } else {
            output.push('.');
        }
    }
    output
}

pub fn next_row(current: &[bool]) -> Vec<bool> {
    let length = current.len();
    let mut next = Vec::with_capacity(length);
    next.push(apply_rule(false, current[0], current[1]));
    for i in 1..(length - 1) {
        next.push(apply_rule(current[i - 1], current[i], current[i + 1]));
    }
    next.push(apply_rule(current[length - 2], current[length - 1], false));

    next
}

pub fn apply_rule(left: bool, middle: bool, right: bool) -> bool {
    (left && middle && !right) ||
        (!left && middle && right) ||
        (left && !middle && !right) ||
        (!left && !middle && right)
}

pub fn rows(start: Vec<bool>, height: usize) -> Vec<Vec<bool>> {
    let mut output = Vec::with_capacity(height);
    output.push(start);
    for i in 0..(height - 1) {
        let next = next_row(&output[i]);
        output.push(next);
    }
    output
}

pub fn print_board(input: &Vec<Vec<bool>>) {
    println!("");
    for row in input {
        println!("{}", unparse(row));
    }
}

pub fn count_safe(input: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for row in input {
        for &tile in row {
            if !tile {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_row() {
        let input = "..^^.";
        let output = vec![false, false, true, true, false];
        assert_eq!(output, parse(input));
    }

    #[test]
    fn it_unparses_row() {
        let output = "..^^.";
        let input = vec![false, false, true, true, false];
        assert_eq!(output, unparse(&input));
    }

    #[test]
    fn it_generates_next_row() {
        let input = parse(".^^.^.^^^^");
        let output = parse("^^^...^..^");
        assert_eq!(output, next_row(&input));
    }

    #[test]
    fn it_applies_rule() {
        assert!(apply_rule(true, true, false));
        assert!(apply_rule(false, true, true));
        assert!(apply_rule(true, false, false));
        assert!(apply_rule(false, false, true));
        assert!(!apply_rule(true, false, true));
        assert!(!apply_rule(false, true, false));
    }

    #[test]
    fn it_generates_n_rows() {
        let start = parse(".^^.^.^^^^");
        let output = vec![
            parse(".^^.^.^^^^"),
            parse("^^^...^..^"),
            parse("^.^^.^.^^."),
            parse("..^^...^^^"),
            parse(".^^^^.^^.^"),
            parse("^^..^.^^.."),
            parse("^^^^..^^^."),
            parse("^..^^^^.^^"),
            parse(".^^^..^.^^"),
            parse("^^.^^^..^^"),
        ];
        let board = rows(start, 10);
        assert_eq!(10, board.len());
        print_board(&board);
        assert_eq!(output, board);
    }

    #[test]
    fn it_counts_safe() {
        let start = parse(".^^.^.^^^^");
        let board = rows(start, 10);
        let safe = count_safe(&board);
        assert_eq!(38, safe);
    }
}
