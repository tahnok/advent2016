use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut machine = Machine::new(&input);
    let extra_disk = "Disc #11 has 11 positions; at time=0, it is at position 0.".parse().unwrap();
    machine.disks.push(extra_disk);
    println!("{}", machine.drop_time());
}

#[derive(Debug)]
pub struct Disk {
    positions: usize,
    current_position: usize,
    start: usize,
}

impl Disk {
    fn tick(&mut self) {
        self.current_position = (self.current_position + 1) % self.positions;
    }

    fn ball_passes(&self) -> bool {
        self.current_position == 0
    }

    fn time_travel(&mut self, time: usize) {
        self.current_position = (self.start + time) % self.positions;
    }
}

#[derive(Debug)]
pub struct DiskErr;

impl FromStr for Disk {
    type Err = DiskErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let positions = parts.nth(3).unwrap().parse().unwrap();
        let current_position = parts.nth(7).unwrap().split(".").next().unwrap().parse().unwrap();

        Ok(Disk {
            positions: positions,
            current_position: current_position,
            start: current_position,
        })
    }
}

pub struct Machine {
    disks: Vec<Disk>,
    time: usize,
}

impl Machine {
    fn new(input: &str) -> Machine {
        let mut disks = vec![];
        for line in input.lines() {
            let machine = line.trim().parse().unwrap();
            disks.push(machine);
        }
        Machine {
            disks: disks,
            time: 0,
        }
    }

    fn tick(&mut self) {
        for disk in self.disks.iter_mut() {
            disk.tick();
        }
        self.time += 1;
    }

    fn drop_ball(&mut self) -> bool {
        for i in 0..self.disks.len() {
            self.tick();
            if !self.disks[i].ball_passes() {
                return false;
            }
        }
        true
    }

    fn drop_time(&mut self) -> usize {
        let mut time = 0;
        loop {
            if self.drop_ball() {
                return time;
            }
            time += 1;
            self.time_travel(time);
        }
    }

    fn time_travel(&mut self, time: usize) {
        self.time = time;
        for disk in self.disks.iter_mut() {
            disk.time_travel(time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_disk_positions() {
        let disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();
        assert_eq!(13, disk.positions);
    }

    #[test]
    fn parses_starting_position() {
        let disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();
        assert_eq!(1, disk.current_position);
    }

    #[test]
    fn disks_tick() {
        let mut disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();
        assert_eq!(1, disk.current_position);
        disk.tick();
        assert_eq!(2, disk.current_position);
    }

    #[test]
    fn disks_tick_over() {
        let mut disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();
        disk.current_position = 12;
        disk.tick();
        assert_eq!(0, disk.current_position);
    }

    #[test]
    fn disk_lets_ball_through() {
        let mut disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();
        disk.current_position = 0;

        assert!(disk.ball_passes());
    }

    #[test]
    fn disk_doesnt_let_ball_through() {
        let disk: Disk = "Disc #1 has 13 positions; at time=0, it is at position 1.".parse().unwrap();

        assert!(!disk.ball_passes());
    }

    fn get_machine() -> Machine {
        Machine::new(
            "Disc #1 has 5 positions; at time=0, it is at position 4.
             Disc #2 has 2 positions; at time=0, it is at position 1."
             )
    }

    #[test]
    fn machine_parses() {
        let m = get_machine();

        assert_eq!(2, m.disks.len());
    }

    #[test]
    fn machine_ticks() {
        let mut m = get_machine();
        assert_eq!(0, m.time);
        m.tick();
        assert_eq!(1, m.time);
    }

    #[test]
    fn machine_ticks_disks() {
        let mut m = get_machine();
        m.tick();
        assert_eq!(0, m.disks[0].current_position);
    }

    #[test]
    fn dropping_ball_takes_time() {
        let mut m = get_machine();
        m.drop_ball();
        assert_eq!(2, m.time)
    }

    #[test]
    fn dropping_ball_doesnt_succeed() {
        let mut m = get_machine();
        let result = m.drop_ball();
        assert!(!result);
    }

    #[test]
    fn dropping_ball_does_succeed() {
        let mut m = get_machine();

        m.time_travel(5);

        let result = m.drop_ball();
        assert!(result);
    }

    #[test]
    fn finds_first_time_to_drop_ball() {
        let mut m = get_machine();
        assert_eq!(5, m.drop_time());
    }
}
