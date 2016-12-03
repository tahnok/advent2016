use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let count = count_valid(&input);
    println!("{}", count);
}

pub fn count_valid(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|triple| is_valid(&triple))
        .collect::<Vec<&Vec<u32>>>()
        .len()
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<u32>>>()
        .chunks(3)
        .flat_map(|chunk| transpose(chunk))
        .collect::<Vec<Vec<u32>>>()
}

pub fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn transpose(input: &[Vec<u32>]) -> Vec<Vec<u32>> {
    vec![
        vec![
            input[0][0],
            input[1][0],
            input[2][0],
        ],
        vec![
            input[0][1],
            input[1][1],
            input[2][1],
        ],
        vec![
            input[0][2],
            input[1][2],
            input[2][2],
        ],
    ]
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
    fn it_parses_line_input() {
        let input = "  810  679   10";
        let expected = vec![810, 679, 10];
        assert_eq!(expected, parse_line(input));
    }

    #[test]
    fn it_parses_input() {
        let input = "101 301 501\n\
                     102 302 502\n\
                     103 303 503\n\
                     201 401 601\n\
                     202 402 602\n\
                     203 403 603\n";
        let output = vec![
            vec![101, 102, 103],
            vec![301, 302, 303],
            vec![501, 502, 503],
            vec![201, 202, 203],
            vec![401, 402, 403],
            vec![601, 602, 603],
        ];

        assert_eq!(output, parse_input(input));
    }

    #[test]
    fn it_transposes() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let output = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9]
        ];

        assert_eq!(output, transpose(&input));
    }

    #[test]
    fn it_counts_valid_triangles() {
        let input = "  810  679   10\n\
                        84  910  149\n\
                       607  425  901\n";
        assert_eq!(1, count_valid(input));
    }
}
