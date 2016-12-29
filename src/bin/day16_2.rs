use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let length = 35651584;
    let checksum = fill_and_checksum(&input.trim(), length);

    println!("{}", checksum);
}


pub fn gen_dragon(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    output.extend_from_slice(input);
    output.push(0);
    output.extend(swap_bits_and_reverse(input));
    output
}

fn swap_bits_and_reverse(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    for bit in input.iter().rev() {
        if bit == &0 {
            output.push(1);
        } else if bit == &1 {
            output.push(0);
        } else {
            println!("{}", bit);
            panic!("don't know how to swap")
        }
    }
    output
}

pub fn checksum(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    let mut iter = input.iter();

    let mut bit1 = iter.next();
    let mut bit2 = iter.next();
    while bit2.is_some() {
        let val1 = bit1.unwrap();
        let val2 = bit2.unwrap();

        if val1 + val2 == 1 {
            output.push(0);
        } else {
            output.push(1);
        }

        bit1 = iter.next();
        bit2 = iter.next();
    }

    if output.len() % 2 == 0 {
        checksum(&output)
    } else {
        output
    }
}

pub fn split(input: &str) -> Vec<u8> {
    input.chars().map(|x| (x as u8) - 48).collect()
}

pub fn fill_and_checksum(input: &str, length: usize) -> String {
    let mut filled = gen_dragon(&split(input));
    while filled.len() < length {
        filled = gen_dragon(&filled);
    }
    let checksum = checksum(&filled[0..length]);
    String::from_utf8(checksum.iter().map(|x| x + 48).collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_expands_1() {
        let input = vec![1];
        let output = vec![1,0,0];
        assert_eq!(output, gen_dragon(&input));
    }

    #[test]
    fn it_expands_0() {
        let input = vec![0];
        let output = vec![0,0,1];
        assert_eq!(output, gen_dragon(&input));
    }

    #[test]
    fn it_expands_hard() {
        let input = vec![1,1,1,1,0,0,0,0,1,0,1,0];
        let output = vec![1,1,1,1,0,0,0,0,1,0,1,0,0,1,0,1,0,1,1,1,1,0,0,0,0];
        assert_eq!(output, gen_dragon(&input));
    }

    #[test]
    fn it_generates_checksum() {
        let input = vec![1,1,0,0,1,0,1,1,0,1,0,0];
        let output = vec![1,0,0];
        assert_eq!(output, checksum(&input));
    }

    #[test]
    fn it_splits() {
        let output = vec![1,0,0,0,0,0,1,1,1,1,0];
        assert_eq!(output, split("10000011110"));
    }

    #[test]
    fn it_fills_to_checksum() {
        assert_eq!("01100", fill_and_checksum("10000", 20));
    }
}
