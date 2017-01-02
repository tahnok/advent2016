use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let blocks = parse(&input);
    let merged_blocks = merge_ranges(blocks);
    println!("{}", merged_blocks[0].1 + 1);
}

pub fn parse_range(input: &str) -> (usize, usize) {
    let mut parts = input.split("-");
    (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
}

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    'outer: for line in input.lines() {
        let (new_min, new_max) = parse_range(line.trim());
        if output.len() == 0 {
            output.push((new_min, new_max));
            continue;
        }
        'inner: for current_range in output.iter_mut() {
            let mut min_in_range = false;
            let mut max_in_range = false;
            if new_min >= current_range.0 && new_min <= current_range.1 {
                if new_max > current_range.1 {
                    current_range.1 = new_max;
                    continue 'outer;
                } else {
                    min_in_range = true;
                }
            }
            if new_max <= current_range.1 && new_max >= current_range.0 {
                if new_min < current_range.0 {
                    current_range.0 = new_min;
                    continue 'outer;
                } else {
                    max_in_range = true;
                }
            }

            if max_in_range && min_in_range {
                continue 'outer;
            }
        }
        output.push((new_min, new_max));
    }
    output.sort();
    output
}

pub fn merge_ranges(input: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    let mut skip_next = 0;
    for (i, x) in input.iter().enumerate() {
        if skip_next > 0{
            skip_next -= 1;
            continue;
        }
        let mut to_push = *x;
        for y in (i + 1)..input.len() {
            if i + y >= input.len() {
                break;
            }
            let next = input[i + y];
            if to_push.1 + 1 == next.0 {
                skip_next += 1;
                to_push.1 = next.1;
            }
        }
        output.push(to_push);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_range() {
        assert_eq!((5,8), parse_range("5-8"));
    }

    #[test]
    fn parses_and_merges_ranges() {
        let input = "5-8\n0-2\n4-7\n0-1";
        let output = vec![
            (0, 2),
            (4, 8),
        ];
        assert_eq!(output, parse(input));
    }

    #[test]
    fn merges_ranges() {
        let input = vec![
            (0,2),
            (3,4),
            (5,6),
            (8, 10),
        ];
        let output = vec![(0,6), (8, 10)];
        assert_eq!(output, merge_ranges(input));
    }
}
