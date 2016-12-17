use std::io;
use std::io::Read;
use std::iter;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let decompressed = decompress(input);
    println!("{}", decompressed.len());
}

pub fn decompress(input: String) -> String {
    let tmp = markers(input.split_whitespace().collect());
    let mut decompressed = tmp.0;
    let mut remaining = tmp.1;
    while !remaining.is_empty() {
        let (x, y) = markers(remaining);
        decompressed = decompressed + &x;
        remaining = y;
    }
    decompressed
}

pub fn markers(input: String) -> (String, String) {
    let mut parts = input.splitn(3, (|c| c == '(' || c == ')'));

    let leading = parts.next().unwrap().to_string();

    let tmp = parts.next();
    if tmp.is_none() {
        return (leading, "".to_string());
    }

    let mut middle = tmp.unwrap().split("x");
    let to_repeat: usize = middle.next().unwrap().parse().unwrap();
    let count: usize = middle.next().unwrap().parse().unwrap();

    let trailing = parts.next().unwrap();

    let repeats: String = iter::repeat(&trailing[..to_repeat]).take(count).collect();

    let end = trailing[to_repeat..].to_string();

    (leading + &repeats, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decompresses_no_markers() {
        assert_eq!("ADVENT", decompress("ADVENT".to_string()));
    }

    #[test]
    fn it_extracts_string_to_repeat() {
        let compress = markers("A(2x5)BC".to_string());
        assert_eq!("ABCBCBCBCBC", compress.0);
    }

    #[test]
    fn it_decompresses_simple_marker() {
        assert_eq!("ABBBBBC", decompress("A(1x5)BC".to_string()));
    }

    #[test]
    fn it_decompresses_more() {
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY".to_string()));
    }

    #[test]
    fn it_decompresses_foo() {
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG".to_string()));
    }
}
