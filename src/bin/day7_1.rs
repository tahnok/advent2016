#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;
use std::io::Read;
use std::str::FromStr;

lazy_static! {
    static ref HYPERNET: Regex = Regex::new(r#"\[(\w+)\]"#).unwrap();
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let sum = IPv7::sum_valid(input.lines().collect::<Vec<&str>>());
    println!("{}", sum);
}

#[derive(Debug)]
pub struct IPv7 {
    hypernet_sequences: Vec<String>,
    address: String,
}

impl IPv7 {
    fn supports_tls(&self) -> bool {
        for sequence in self.hypernet_sequences.iter() {
            if has_abba(&sequence) {
                return false;
            }
        }
        has_abba(&self.address)
    }

    fn sum_valid(ips: Vec<&str>) -> u32 {
        let mut sum = 0;
        for ip in ips {
            let parsed: IPv7 = ip.parse().unwrap();
            if parsed.supports_tls() {
                sum += 1;
            }
        }
        sum
    }
}

impl FromStr for IPv7 {
    type Err = IPv7Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut captures = vec![];
        for capture in HYPERNET.captures_iter(s) {
            captures.push(capture.at(1).unwrap_or("").to_string());
        }
        Ok(IPv7{
            hypernet_sequences: captures,
            address: s.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct IPv7Err {
}

pub fn has_abba(input: &str) -> bool {
    let raw = input.bytes().collect::<Vec<u8>>();
    for i in 3..raw.len() {
        if raw[i - 3] == raw[i - 2] {
            continue;
        }
        if raw[i - 3] == raw[i] && raw[i - 2] == raw[i - 1] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_abbas() {
        assert!(has_abba("abba"));
        assert!(has_abba("xabba"));
        assert!(has_abba("xyyx"));
        assert!(has_abba("ioxxoj"));
    }

    #[test]
    fn it_doesnt_detect_no_abbas() {
        assert!(!has_abba("aaaa"));
        assert!(!has_abba("abcd"));
    }

    #[test]
    fn it_parses_ipv7() {
        let _: IPv7 = "abba[mnop]qrst".parse().unwrap();
    }

    #[test]
    fn it_extracts_hypernet_sequences() {
        let ipv7: IPv7 = "onmmhtsykubbpdiqvjm[kbfbiyjyuzmemaomkwa]prqwqocsihfnslooel[hysggeprqecalydywlk]taghiwhgnujsduhnffu[ibpvowghgttfsvt]wcajwcxhcriflxi".parse().unwrap();
        assert_eq!(vec!["kbfbiyjyuzmemaomkwa", "hysggeprqecalydywlk", "ibpvowghgttfsvt"], ipv7.hypernet_sequences);
    }

    #[test]
    fn it_extracts_address() {
        let ipv7: IPv7 = "abba[mnop]qrst".parse().unwrap();
        assert_eq!("abba[mnop]qrst", ipv7.address);
    }

    #[test]
    fn it_knows_when_ipv7_supports_tls() {
        let ipv7: IPv7 = "abba[mnop]qrst".parse().unwrap();
        assert!(ipv7.supports_tls());

        let ipv7_2: IPv7 = "ioxxoj[asdfgh]zxcvbn".parse().unwrap();
        assert!(ipv7_2.supports_tls());
    }

    #[test]
    fn it_knows_when_ipv7_doesnt_support_tls() {
        let ipv7_1: IPv7 = "abcd[bddb]xyyx".parse().unwrap();
        assert!(!ipv7_1.supports_tls());

        let ipv7_2: IPv7 = "aaaa[qwer]tyui".parse().unwrap();
        assert!(!ipv7_2.supports_tls());
    }

    #[test]
    fn it_sums_valid() {
        let ips = vec![
            "abba[mnop]qrst",
            "abcd[bddb]xyyx",
            "aaaa[qwer]tyui",
            "ioxxoj[asdfgh]zxcvbn",
        ];
        assert_eq!(2, IPv7::sum_valid(ips));
    }
}
