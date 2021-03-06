extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

fn main() {
    let mut hasher = Hasher::new("yjdafjpo");
    for i in 0..64 {
        println!("{}: {}", i, hasher.next_key());
    }
}

pub fn has_triple(chars: &[char]) -> Option<char> {
    for x in 0..30 {
        if chars[x] == chars[x + 1] && chars[x + 1] == chars[x + 2] {
            return Some(chars[x])
        }
    }
    None
}

pub fn has_quintuple(chars: &[char], contains: char) -> bool {
    let mut count = 0;
    for x in chars {
        if *x == contains {
            if count == 4 {
                return true;
            }
            count += 1;
        } else {
            count = 0;
        }
    }
    false
}

fn hash(key: &str, index: usize, hasher: &mut Md5) -> Vec<char> {
    hasher.reset();

    hasher.input_str(key);
    hasher.input_str(&index.to_string());

    let mut stretching = hasher.result_str();

    for _ in 0..2016 {
        hasher.reset();
        hasher.input_str(&stretching);
        stretching = hasher.result_str();
    }

    stretching.chars().collect()
}

pub struct Hasher {
    key: String,
    digest: Md5,
    index: usize,
    lookup: HashMap<usize, Vec<char>>,
}

impl Hasher {
    fn new(key: &str) -> Hasher {
        Hasher {
            key: key.to_string(),
            digest: Md5::new(),
            index: 0,
            lookup: HashMap::new(),
        }
    }

    fn get(&mut self, index: usize) -> Vec<char> {
        if self.lookup.contains_key(&index) {
            return self.lookup.get(&index).unwrap().clone()
        }
        self.lookup.entry(index).or_insert(
            hash(&self.key, index, &mut self.digest)
            ).clone()
    }

    fn next_triple(&mut self) -> (usize, char) {
        loop {
            let index = self.index;
            let hash = self.get(index);
            self.index += 1;
            match has_triple(&hash) {
                Some(matching) => return (index, matching),
                None => (),
            }
        }
    }

    fn next_key(&mut self) -> usize {
        loop {
            let (index, matching) = self.next_triple();
            if self.quintuple_in_next_thousand(index, matching) {
                return index;
            }
        }
    }

    fn quintuple_in_next_thousand(&mut self, index: usize, matching: char) -> bool {
        for x in 1..1001 {
            let hash = self.get(x + index);
            if has_quintuple(&hash, matching) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crypto::md5::Md5;
    use crypto::digest::Digest;
    use std::time::Instant;
    use super::*;

    fn hash(input: &str) -> Vec<char> {
        let mut hasher = Md5::new();
        hasher.input_str(input);
        hasher.result_str().chars().collect()
    }

    #[test]
    fn detects_triples() {
        let hash = hash("abc18");
        assert_eq!(Some('8'), has_triple(&hash));
    }

    #[test]
    fn detects_no_triples() {
        let hash = hash("abc17");
        assert_eq!(None, has_triple(&hash));
    }

    #[test]
    fn detects_quintuple() {
        let hash = hash("abc816");
        assert!(has_quintuple(&hash, 'e'));
    }

    #[test]
    fn detects_no_quintuple() {
        let hash = hash("abc815");
        assert!(!has_quintuple(&hash, 'e'));
    }

    #[test]
    fn it_hashes() {
        let mut hasher = Hasher::new("abc");
        let expect = "a107ff634856bb300138cac6568c0f24".to_string();
        let result: String = hasher.get(0).into_iter().collect();
        assert_eq!(expect, result);
    }

    #[ignore]
    #[test]
    fn it_times_hashes() {
        let mut hasher = Hasher::new("abc");
        for _ in 0..10 {
            let mut start = Instant::now();
            let _ = hasher.get(0);
            let elapsed = start.elapsed();
            println!("{}", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
        }

        assert!(false);
    }

    #[test]
    fn it_finds_triples() {
        let mut hasher = Hasher::new("abc");
        assert_eq!((5, '2'), hasher.next_triple());
        assert_eq!((10, 'e'), hasher.next_triple());
    }


    #[test]
    fn it_finds_key() {
        let mut hasher = Hasher::new("abc");
        assert_eq!(10, hasher.next_key());
    }
}
