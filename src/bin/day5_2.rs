extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{:08x}", generate(input.trim()));
}

pub fn generate(input: &str) -> u32 {
    let mut hasher = Md5::new();
    let mut result: [i8; 8] = [-1; 8];
    let mut hacks = "".to_string();
    for x in 0..u32::max_value() {
        let hashed = hash(&mut hasher, input, x);
        if special(hashed) {
            let (index, value) = extract(hashed);
            set(&mut result, index, value);
            hacks = fancy_print(&result, hacks);
            // println!("{:?}", result);
            if is_ready(&result) {
                break;
            }
        }
    }
    join(result)
}

pub fn hash(hasher: &mut Md5, input: &str, offset: u32) -> [u8; 16] {
    hasher.reset();
    hasher.input(input.as_bytes());
    hasher.input(offset.to_string().as_bytes());
    let mut output = [0; 16];
    hasher.result(&mut output);
    output
}

pub fn special(input: [u8; 16]) -> bool {
    input[0] == 0 &&
    input[1] == 0 &&
    input[2] <= 0xf
}

pub fn extract(input: [u8; 16]) -> (u8, u8) {
    (input[2] & 0xf, input[3].wrapping_shr(4))
}

pub fn join(input: [i8; 8]) -> u32 {
    let mut result: u32 = 0;
    for i in 0..8 {
        result = result << 4;
        result = result + (input[i] as u32);
    }
    result
}

pub fn set(input: &mut [i8; 8], index: u8, value: u8) {
    if index < 8 {
        let cast_index = index as usize;
        if input[cast_index] == -1 {
            input[cast_index] = value as i8;
        }
    }
}

pub fn is_ready(input: &[i8; 8]) -> bool {
    for x in input {
        if *x == -1 {
            return false
        }
    }
    true
}

fn fancy_print(input: &[i8; 8], hacks: String) -> String {
    let mut foo = vec![];
    for x in input {
        if *x == -1 {
            foo.push("_".to_string());
        } else {
            foo.push(format!("{:x}", x));
        }
    }
    let result = foo.join("");
    if result != hacks {
        println!("{}", result);
    }
    result
}

#[cfg(test)]
mod tests {
    use crypto::md5::Md5;
    use super::*;

    #[test]
    fn it_computes_a_hash() {
        let mut hasher = Md5::new();
        let output = [124, 106, 24, 11, 54, 137, 106, 10, 140, 2, 120, 126, 234, 251, 14, 76];
        assert_eq!(output, hash(&mut hasher, "password", 1));
    }

    #[test]
    fn it_detects_special_hashes() {
        let input = [0, 0, 15, 99, 90, 167, 101, 214, 29, 131, 39, 222, 184, 130, 207, 153];
        assert!(special(input));
    }

    #[test]
    fn it_rejetects_not_special_hashes() {
        let input = [95, 77, 204, 59, 90, 167, 101, 214, 29, 131, 39, 222, 184, 130, 207, 153];
        assert!(!special(input));
    }

    #[test]
    fn it_finds_special_words() {
        let mut hasher = Md5::new();
        let hashed = hash(&mut hasher, "abc", 3231929);
        println!("{:?}", hashed);
        assert!(special(hashed));
    }

    #[test]
    fn it_extracts_the_char_and_position() {
        let input = [0, 0, 01, 0xf0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!((0x1, 0xf), extract(input));

    }

    #[test]
    fn it_joins() {
        let input1 = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(0x12345678, join(input1));
        let input2 = [0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8];
        assert_eq!(0xfedcba98, join(input2));
    }

    #[test]
    fn it_sets_output() {
        let mut output: [i8; 8] = [-1; 8];
        set(&mut output, 7, 9);
        assert_eq!([-1, -1, -1, -1, -1, -1, -1, 9], output);
    }

    #[test]
    fn it_sets_ignoring_invalid_index() {
        let mut output: [i8; 8] = [-1; 8];
        set(&mut output, 9, 9);
        assert_eq!([-1, -1, -1, -1, -1, -1, -1, -1], output);
    }

    #[test]
    fn it_sets_ignore_already_set() {
        let mut output: [i8; 8] = [-1; 8];
        output[4] = 4;
        set(&mut output, 4, 9);
        assert_eq!([-1, -1, -1, -1, 4, -1, -1, -1], output);
    }

    #[test]
    fn it_knows_when_output_is_ready() {
        let input: [i8; 8] = [1; 8];
        assert!(is_ready(&input));
    }

    #[test]
    fn it_knows_when_output_is_not_ready() {
        let mut input: [i8; 8] = [1; 8];
        input[2] = -1;
        assert!(!is_ready(&input));
    }


}
