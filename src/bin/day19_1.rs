
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
        while self.available.len() > 1 {
            let mut current_gen = Vec::with_capacity(self.available.len() / 2);
            let drop_head = self.available.len() % 2 == 1;
            for (i, x) in self.available.iter().enumerate() {
                if i % 2 == 0 {
                    current_gen.push(*x)
                }
            }
            if drop_head {
                current_gen.remove(0);
            }
            self.available = current_gen;
        }
        self.available[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winning_elf() {
        let mut ring = Ring::new(5);
        assert_eq!(3, ring.winner());
    }
}
