use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {

    // let dest = (31, 39);
    let fav_number = 1350;
    let steps = 50;
    println!("{}", max_loc_visited(steps, fav_number));
}

#[derive(Debug, PartialEq)]
pub enum Feature {
    Wall,
    Open,
}

pub fn kind(point: (usize, usize), fav_number: usize) -> Feature {
    let (x, y) = point;
    let mut value = (x*x) + (3*x) + (2*x*y) + y + (y*y);
    value += fav_number;
    if value.count_ones() % 2 == 0 {
        Feature::Open
    } else {
        Feature::Wall
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Step {
    loc: (usize, usize),
    distance: usize,
}

pub fn neighbours(step: Step, fav_number: usize) -> Vec<Step> {
    let mut output = vec![];
    let point = step.loc;
    let distance = step.distance + 1;

    let north = (point.0, point.1 + 1);
    if kind(north, fav_number) == Feature::Open {
        output.push(Step { loc: north, distance: distance });
    }

    let east = (point.0 + 1, point.1);
    if kind(east, fav_number) == Feature::Open {
        output.push(Step { loc: east, distance: distance });
    }

    if point.1 > 0 {
        let south = (point.0, point.1 - 1);
        if kind(south, fav_number) == Feature::Open {
            output.push(Step { loc: south, distance: distance });
        }
    }

    if point.0 > 0 {
        let west = (point.0 - 1, point.1);
        if kind(west, fav_number) == Feature::Open {
            output.push(Step { loc: west, distance: distance });
        }
    }

    output
}

pub fn add_unvisited(step: Step, fav_number: usize, to_visit: &mut VecDeque<Step>, visited: &HashSet<(usize, usize)>) {
    let maybe = neighbours(step, fav_number);
    for step in maybe {
        if !visited.contains(&step.loc) {
            to_visit.push_back(step);
        }
    }
}

pub fn max_loc_visited(max_step: usize, fav_number: usize) -> usize {
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    let start = Step { loc: (1, 1), distance: 0};
    visited.insert((1,1));
    add_unvisited(start, fav_number, &mut to_visit, &visited);

    while to_visit.len() > 0 {
        let current = to_visit.pop_front().unwrap();
        if current.distance <= max_step {
            visited.insert(current.loc);
            add_unvisited(current, fav_number, &mut to_visit, &visited);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn it_detects_walls() {
        assert_eq!(Feature::Wall, kind((1,0), 10));
        assert_eq!(Feature::Wall, kind((2,1), 10));
    }

    #[test]
    fn it_detects_open() {
        assert_eq!(Feature::Open, kind((1,1), 10));
    }

    #[test]
    fn it_generates_possible_neighbours() {
        let output = vec![
            Step { loc: (3, 3), distance: 1},
            Step { loc: (4, 2), distance: 1},
            Step { loc: (3, 1), distance: 1},
            Step { loc: (2, 2), distance: 1},
        ];
        let start = Step { loc: (3, 2), distance: 0};
        assert_eq!(output, neighbours(start, 10));
    }

    #[test]
    fn it_generates_possible_neighbours_1_1() {
        let output = vec![
            Step { loc: (1, 2), distance: 1},
            Step { loc: (0, 1), distance: 1},
        ];
        let start = Step { loc: (1, 1), distance: 0};
        assert_eq!(output, neighbours(start, 10));
    }

    #[test]
    fn it_generates_possible_neighbours_next_to_walls() {
        let output = vec![
            Step { loc: (0, 1), distance: 1},
        ];
        let start = Step { loc: (0, 0), distance: 0};
        assert_eq!(output, neighbours(start, 10));
    }

    #[test]
    fn it_adds_unvisited_neighbours() {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert((1,2));

        add_unvisited(Step { loc: (1,1), distance: 0}, 10, &mut to_visit, &visited);

        assert_eq!(to_visit.pop_front(), Some(Step { loc: (0,1), distance: 1}));
        assert_eq!(to_visit.len(), 0);
    }

    #[ignore]
    #[test]
    fn it_prints_map() {
        println!("");
        for y in 0..7 {
            for x in 0..10 {
                let feature = match kind((x,y), 10) {
                    Feature::Wall => "#",
                    Feature::Open => ".",
                };
                print!("{}", feature);
            }
            println!("");
        }
        assert!(false);
    }

    #[test]
    fn it_works() {
        assert_eq!(5, max_loc_visited(2, 10));
    }

}
