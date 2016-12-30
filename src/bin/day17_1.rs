extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let path = find_path(&input);
    println!("{}", path.steps);
}

pub struct Path {
    steps: String,
    passcode: String,
    hasher: Md5,
    x: usize,
    y: usize,
}

impl Path {
    fn new(passcode: &str) -> Path {
        Path {
            steps: String::new(),
            passcode: passcode.to_string(),
            hasher: Md5::new(),
            x: 0,
            y: 3,
        }
    }

    fn hash(&mut self) -> Vec<u8> {
        self.hasher.reset();
        self.hasher.input_str(&self.passcode);
        self.hasher.input_str(&self.steps);

        self.hasher.result_str()[0..4]
            .chars()
            .map(|x| u8::from_str_radix(&x.to_string(), 16).unwrap())
            .collect()
    }

    fn open_doors(&mut self) -> Vec<char> {
        let hashed = self.hash();
        let mut result = vec![];

        if hashed[0] > 10 && self.y < 3 {
            result.push('U');
        }
        if hashed[1] > 10 && self.y > 0 {
            result.push('D');
        }
        if hashed[2] > 10 && self.x > 0{
            result.push('L');
        }
        if hashed[3] > 10 && self.x < 3 {
            result.push('R');
        }

        result
    }

    fn step(&self, direction: char) -> Path {
        let mut new = self.clone();
        new.steps.push(direction);
        match direction {
            'U' => new.y = self.y + 1,
            'D' => new.y = self.y - 1,
            'L' => new.x = self.x - 1,
            'R' => new.x = self.x + 1,
            _ => panic!("unknown direction"),
        }
        new
    }

    fn won(&self) -> bool {
        self.x == 3 && self.y == 0
    }
}

impl Clone for Path {
    fn clone(&self) -> Path {
        Path {
            steps: self.steps.clone(),
            passcode: self.passcode.clone(),
            hasher: Md5::new(),
            x: self.x,
            y: self.y,
        }
    }
}

impl fmt::Debug for Path {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Path {{ x: {}, y: {}, steps: {}, passcode: {} }}", self.x, self.y, self.steps, self.passcode)
        }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.passcode == other.passcode &&
            self.steps == other.steps
    }
}

pub fn add_possible(location: &mut Path, to_visit: &mut VecDeque<Path>) {
    for x in location.open_doors() {
        to_visit.push_back(location.step(x));
    }
}

pub fn find_path(passcode: &str) -> Path {
    let mut to_visit = VecDeque::new();

    let start = Path::new(passcode);
    to_visit.push_back(start);
    while to_visit.len() > 0 {
        let mut current = to_visit.pop_front().unwrap();
        if current.won() {
            return current;
        }
        add_possible(&mut current, &mut to_visit);
    }
    panic!("no route found");
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn makes_path() {
        let p = Path::new("ihgpwlah");
        assert_eq!("", p.steps);
        assert_eq!("ihgpwlah", p.passcode);
    }

    #[test]
    fn it_hashes() {
        let mut p = Path::new("hijkl");

        assert_eq!(vec![0xc, 0xe, 0xd, 0x9], p.hash());
    }

    #[test]
    fn it_hashes_with_steps() {
        let mut p = Path::new("hijkl");
        p.steps = "DR".to_string();

        assert_eq!(vec![5,7,4,5], p.hash());
    }

    #[test]
    fn it_generates_open_doors_at_start() {
        let mut p = Path::new("hijkl");
        assert_eq!(vec!['D'], p.open_doors());
    }

    #[test]
    fn it_generates_open_doors_when_stuck() {
        let mut p = Path::new("hijkl");
        p.steps = "DUR".to_string();
        let output: Vec<char> = vec![];
        assert_eq!(output, p.open_doors());
    }

    #[test]
    fn it_steps() {
        let start = Path::new("hijkl");
        let stepped = start.step('D');
        assert_eq!(2, stepped.y);
        assert_eq!(0, stepped.x);
        assert_eq!("D", stepped.steps);
    }

    #[test]
    fn it_multi_steps() {
        let path = Path::new("hijkl").step('D').step('R');
        assert_eq!("DR", path.steps);
    }

    #[test]
    fn it_knows_when_not_won() {
        let path = Path::new("hijkl");
        assert!(!path.won());
    }

    #[test]
    fn it_knows_when_won() {
        let mut path = Path::new("hijkl");
        path.y = 0;
        path.x = 3;
        assert!(path.won());
    }

    #[test]
    fn it_finds_possible_paths() {
        let mut start = Path::new("hijkl");

        let mut to_visit = VecDeque::new();
        add_possible(&mut start, &mut to_visit);

        let next = start.step('D');
        assert_eq!(Some(next), to_visit.pop_front());
        assert_eq!(0, to_visit.len());
    }

    #[should_panic]
    #[test]
    fn it_panics_when_no_route() {
        find_path("hijkl");
    }

    #[test]
    fn it_finds_shortest_path_easy() {
        let path = find_path("ihgpwlah");
        assert_eq!("DDRRRD", path.steps);
    }

    #[test]
    fn it_finds_shortest_path_medium() {
        let path = find_path("kglvqrro");
        assert_eq!("DDUDRLRRUDRD", path.steps);
    }

    #[test]
    fn it_finds_shortest_path_hard() {
        let path = find_path("ulqzkmiv");
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", path.steps);
    }
}
