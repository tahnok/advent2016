use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let count = count_valid(&input);
    println!("{}", count);
}

pub fn count_valid(input: &str) -> usize {
    input.lines()
        .map(|line| parse(line))
        .filter(|triple| is_valid(&triple))
        .collect::<Vec<Vec<u32>>>()
        .len()
}

pub fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn is_valid(sides: &Vec<u32>) -> bool {
    let mut sorted_sides = sides.clone();
    sorted_sides.sort();

    let a = sorted_sides[0];
    let b = sorted_sides[1];
    let c = sorted_sides[2];
    (a + b) > c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_valid_triangles() {
        let triangle = vec![5, 10, 12];
        assert!(is_valid(&triangle));
    }

    #[test]
    fn it_detects_invalid_triangles() {
        let triangle = vec![5, 10, 25];
        assert!(!is_valid(&triangle));
    }

    #[test]
    fn it_detects_valid_triangles_sides_unordered() {
        let triangle = vec![10, 12, 5];
        assert!(is_valid(&triangle));
    }

    #[test]
    fn it_detects_invalid_triangles_sides_unordered() {
        let triangle = vec![25, 10, 5];
        assert!(!is_valid(&triangle));
    }

    #[test]
    fn it_parses_input() {
        let input = "  810  679   10";
        let expected = vec![810, 679, 10];
        assert_eq!(expected, parse(input));
    }

    #[test]
    fn it_counts_valid_triangles() {
        let input = "  810  679   10\n\
                        84  910  149\n\
                       607  425  901\n\
                       556  616  883";
        assert_eq!(2, count_valid(input));
    }
}
