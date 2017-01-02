use std::collections::HashSet;

fn main() {
    let num_elves = 3014387;
    let mut ring = Ring::new(num_elves);
    let winner = ring.winner();
    println!("{}", winner);
}

pub struct Ring {
    available: Vec<usize>,
}

impl Ring {
    fn new(size: usize) -> Ring {
        let mut available = Vec::with_capacity(size);
        for x in 0..size {
            available.push(x + 1);
        }
        Ring {
            available: available,
        }
    }

    fn winner(&mut self) -> usize {
        let mut removed = HashSet::new();
        for (index, x) in self.available.iter().enumerate() {
            if removed.contains(x) {
                continue;
            }
            let offset = ((self.available.len() - removed.len()) / 2 + index) % self.available.len();
            println!("{:?}", self.available[offset]);
            removed.insert(self.available[offset]);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winning_elf() {
        let mut ring = Ring::new(5);
        assert_eq!(2, ring.winner());
    }
}
