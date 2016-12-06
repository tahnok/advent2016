extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;


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

pub fn extract(input: [u8; 16]) -> u8 {
    input[2] & 0xf
}

pub fn join(input: [u8; 8]) -> u32 {
    let mut result: u32 = 0;
    for i in 0..8 {
        result = result << 4;
        result = result + (input[i] as u32);
    }
    result
}

pub fn generate(input: &str) -> u32 {
    let mut found = 0;
    let mut hasher = Md5::new();
    let mut result: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    for x in 0..u32::max_value() {
        let hashed = hash(&mut hasher, input, x);
        if special(hashed) {
            result[found] = extract(hashed);
            found = found + 1;
            if found > 8 {
                break;
            }
        }
    }
    join(result)
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
    fn it_extracts_the_sixth_char() {
        let input = [95, 77, 204, 59, 90, 167, 101, 214, 29, 131, 39, 222, 184, 130, 207, 153];
        assert_eq!(0xc, extract(input));

    }

    #[test]
    fn it_joins() {
        let input1 = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(0x12345678, join(input1));
        let input2 = [0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8];
        assert_eq!(0xfedcba98, join(input2));
    }

    #[test]
    fn it_generates_a_password() {
        assert_eq!(0x18f47a30, generate("abc"))
    }
}
