use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let ranges = parse_ranges(&input);
    let sorted_ranges = merge_range(&ranges);
    let cleaned_ranges = merge_range(&sorted_ranges); //oops
    let count = count_allowed(&cleaned_ranges, 4294967295);
    println!("{}", count);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range {
            start: start,
            end: end,
        }
    }
}

impl Ord for Range {
        fn cmp(&self, other: &Range) -> Ordering {
            self.start.cmp(&other.start)
        }
}

impl PartialOrd for Range {
        fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
            Some(self.cmp(other))
        }
}

pub fn parse_range(input: &str) -> Range {
    let mut parts = input.split("-");
    Range {
        start: parts.next().unwrap().parse().unwrap(),
        end: parts.next().unwrap().parse().unwrap(),
    }
}

pub fn parse_ranges(input: &str) -> Vec<Range> {
    let mut output = Vec::new();
    for line in input.trim().lines() {
        output.push(parse_range(line));
    }
    output
}

pub fn merge_range(input: &[Range]) -> Vec<Range> {
    let mut output = Vec::new();
    'outer: for range in input {
        for existing_range in output.iter_mut() {
            match compare(existing_range, range) {
                RangeResult::Inclusive => continue 'outer,
                RangeResult::LargerEnd => {
                    existing_range.end = range.end;
                    continue 'outer;
                },
                RangeResult::SmallerStart => {
                    existing_range.start = range.start;
                    continue 'outer;
                },
                RangeResult::SuperSet => {
                    existing_range.start = range.start;
                    existing_range.end = range.end;
                    continue 'outer;
                },
                RangeResult::Exclusive => (),
            }
        }
        output.push(*range);
    }
    output.sort();
    output
}

#[derive(Debug, PartialEq)]
pub enum RangeCheck {
    Inside,
    Before,
    After,
}

pub fn in_range(range: &Range, to_check: usize) -> RangeCheck {
    if range.start > to_check {
        return RangeCheck::Before
    }
    if range.end < to_check {
        return RangeCheck::After;
    }
    RangeCheck::Inside
}

#[derive(Debug, PartialEq)]
pub enum RangeResult {
    Inclusive,
    Exclusive,
    LargerEnd,
    SmallerStart,
    SuperSet
}

pub fn compare(left: &Range, right: &Range) -> RangeResult {
    let start_status = in_range(left, right.start);
    let end_status = in_range(left, right.end);
    if start_status == RangeCheck::Inside && end_status == RangeCheck::Inside {
        return RangeResult::Inclusive;
    }
    if start_status == RangeCheck::Before && end_status == RangeCheck::Inside {
        return RangeResult::SmallerStart;
    }
    if start_status == RangeCheck::Inside && end_status == RangeCheck::After {
        return RangeResult::LargerEnd;
    }
    if start_status == RangeCheck::Before && end_status == RangeCheck::After {
        return RangeResult::SuperSet;
    }
    RangeResult::Exclusive
}

pub fn count_allowed(ranges: &[Range], max: usize) -> usize {
    let mut count = 0;
    if ranges[0].start != 0 {
        count += ranges[0].start;
    }

    for i in 1..ranges.len() {
        count += ranges[i].start - ranges[i - 1].end - 1;
    }

    let last = ranges[ranges.len() - 1];
    if last.end != max {
        count += max - last.end;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_range() {
        assert_eq!(Range::new(5,8), parse_range("5-8"));
    }

    #[test]
    fn parses_range_list() {
        let input = "5-8\n0-2\n4-7\n";
        let expected = vec![Range::new(5,8), Range::new(0,2), Range::new(4,7)];
        assert_eq!(expected, parse_ranges(input));
    }

    #[test]
    fn merges_ranges() {
        let input = vec![Range::new(6,8), Range::new(0,2), Range::new(5,7), Range::new(0,1), Range::new(4, 5)];
        let output = vec![Range::new(0,2), Range::new(4,8)];
        assert_eq!(output, merge_range(&input));
    }

    #[test]
    fn inside_range() {
        assert_eq!(RangeCheck::Inside, in_range(&Range::new(0, 4), 2));
        assert_eq!(RangeCheck::Inside, in_range(&Range::new(0, 4), 0));
        assert_eq!(RangeCheck::Inside, in_range(&Range::new(0, 4), 4));
    }

    #[test]
    fn before_range() {
        assert_eq!(RangeCheck::Before, in_range(&Range::new(2, 4), 1));
    }

    #[test]
    fn after_range() {
        assert_eq!(RangeCheck::After, in_range(&Range::new(2, 4), 5));
    }

    #[test]
    fn compares_inclusive_ranges() {
        assert_eq!(RangeResult::Inclusive, compare(&Range::new(0,4), &Range::new(1,2)));
    }

    #[test]
    fn compares_exlusive_range() {
        assert_eq!(RangeResult::Exclusive, compare(&Range::new(0,4), &Range::new(5, 8)));
    }

    #[test]
    fn compares_smaller_start() {
        assert_eq!(RangeResult::SmallerStart, compare(&Range::new(3, 5), &Range::new(2, 4)));
    }

    #[test]
    fn compare_end_larger_range() {
        assert_eq!(RangeResult::LargerEnd, compare(&Range::new(0, 2), &Range::new(1, 3)));
    }

    #[test]
    fn compares_superset_range() {
        assert_eq!(RangeResult::SuperSet, compare(&Range::new(3, 4), &Range::new(1, 5)));
    }

    #[test]
    fn counts_allowed() {
        let input = vec![Range::new(0,2), Range::new(4, 8)];
        let max = 9;
        assert_eq!(2, count_allowed(&input, max));
    }
}
