use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let decompressed = decompressed_length(&strip_whitespace(input));
    println!("{}", decompressed);
}

fn strip_whitespace(input: String) -> String {
    input.split_whitespace().collect()
}

pub fn decompressed_length(input: &str) -> usize {
    let mut parts = input.splitn(3, (|c| c == '(' || c == ')'));

    let leading = parts.next().unwrap().len();

    let tmp = parts.next();
    if tmp.is_none() {
        return leading;
    }

    let mut middle = tmp.unwrap().split("x");
    let to_repeat: usize = middle.next().unwrap().parse().unwrap();
    let count: usize = middle.next().unwrap().parse().unwrap();

    let trailing = parts.next().unwrap();

    leading + (count * decompressed_length(&trailing[..to_repeat])) + decompressed_length(&trailing[to_repeat..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_with_no_markers() {
        assert_eq!(6, decompressed_length("ADVENT"));
    }

    #[test]
    fn it_counts_with_one_marker() {
        assert_eq!(7, decompressed_length("A(1x5)BC"));
    }

    #[test]
    fn it_counts_with_repeated_markers() {
        assert_eq!(241920, decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    }

    #[test]
    fn it_still_counts_with_nested() {
        assert_eq!(445, decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }
}
