#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

lazy_static! {
    static ref SECTOR_RE: Regex = Regex::new(r#"(?P<name>.+)-(?P<sid>\d+)\[(?P<cs>[a-z]+)\]"#).unwrap();
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let sum = RoomCode::sum_valid(input.lines().collect::<Vec<&str>>());
    println!("{}", sum);
}

#[derive(Debug)]
pub struct RoomCode {
    checksum: String,
    sector_id: u32,
    name: String,
}

impl RoomCode {
    fn common_chars(&self) -> Vec<char> {
        let chars = self.name
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<Vec<char>>();

        let mut pairs = HashMap::new();
        for x in &chars {
            let entry = pairs.entry(x).or_insert(0);
            *entry += 1;
        }
        let mut sorted_pairs = vec![];
        for (key, value) in pairs {
            sorted_pairs.push((key, value));
        }
        sorted_pairs.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.0.cmp(&b.0),
            }
        });

        let mut result = vec![];
        for x in 0..5 {
            result.push(*sorted_pairs[x].0);
        }
        result
    }

    fn computed_checksum(&self) -> String {
        self.common_chars()
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
    }

    fn is_valid(&self) -> bool {
        self.computed_checksum() == self.checksum
    }

    fn sum_valid(codes: Vec<&str>) -> u32 {
        let mut sum = 0;
        for raw_code in codes {
            let code: RoomCode = raw_code.parse().unwrap();
            if code.is_valid() {
                sum = sum + code.sector_id;
            }

        }
        sum
    }
}

impl FromStr for RoomCode {
    type Err = ParseSectorIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = SECTOR_RE.captures(s).unwrap();
        Ok(RoomCode{
            checksum: captures.name("cs").unwrap().to_string(),
            sector_id: captures.name("sid").unwrap().parse().unwrap(),
            name: captures.name("name").unwrap().to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ParseSectorIdError {
    InvalidFormat
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_from_str() {
        let _: RoomCode = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
    }

    #[test]
    fn it_extracts_checksum() {
        let code: RoomCode = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert_eq!("abxyz", code.checksum);
    }

    #[test]
    fn it_extracts_sector_id() {
        let code: RoomCode = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert_eq!(123, code.sector_id);
    }

    #[test]
    fn it_extracts_name() {
        let code: RoomCode = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert_eq!("aaaaa-bbb-z-y-x", code.name);
    }

    #[test]
    fn it_extracts_most_common_chars_easy() {
        let expteced = vec!['p', 'b', 'x', 'y', 'z'];
        let code: RoomCode = "ppppp-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert_eq!(expteced, code.common_chars());
    }

    #[test]
    fn it_extracts_most_common_chars_hard() {
        let expteced = vec!['a', 'b', 'c', 'd', 'e'];
        let code: RoomCode = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
        assert_eq!(expteced, code.common_chars());
    }

    #[test]
    fn it_computes_checksum() {
        let expteced = "abcde";
        let code: RoomCode = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
        assert_eq!(expteced, code.computed_checksum());
    }

    #[test]
    fn it_validates_valid_checksum() {
        let code1: RoomCode = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
        assert!(code1.is_valid());
        let code2: RoomCode = "ppppp-bbb-z-y-x-123[pbxyz]".parse().unwrap();
        assert!(code2.is_valid());
        let code3: RoomCode = "not-a-real-room-404[oarel]".parse().unwrap();
        assert!(code3.is_valid());
    }

    #[test]
    fn it_doesnt_validates_invalid_checksum() {
        let code2: RoomCode = "totally-real-room-200[decoy]".parse().unwrap();
        assert!(!code2.is_valid());
    }

    #[test]
    fn it_sums_valid_sector_ids() {
        let codes = vec![
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];
        assert_eq!(1514, RoomCode::sum_valid(codes));
    }


}
