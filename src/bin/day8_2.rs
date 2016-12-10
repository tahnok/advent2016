#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;
use std::io::Read;

lazy_static! {
    static ref RECT: Regex = Regex::new(r#"rect (\d+)x(\d+)"#).unwrap();
    static ref ROTATE_COLUMN: Regex = Regex::new(r#"rotate column x=(\d+) by (\d+)"#).unwrap();
    static ref ROTATE_ROW: Regex = Regex::new(r#"rotate row y=(\d+) by (\d+)"#).unwrap();
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut display = Display::new();
    for instruction in input.lines() {
        display.parse(instruction);
    }
    println!("{}", display.print());
}

pub struct Display {
    pixels: [[bool; 50]; 6],
}

impl Display {
    pub fn new() -> Display {
        Display {
            pixels: [[false; 50]; 6],
        }
    }

    pub fn on_pixels(&self) -> u32 {
        let mut count = 0;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                if *pixel {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn print(&self) -> String {
        let mut display = vec![];
        for row in self.pixels.iter() {
            let mut display_row = vec![];
            for pixel in row.iter() {
                if *pixel {
                    display_row.push("#");
                } else {
                    display_row.push(".");
                }
            }
            display.push(display_row.join(""));
        }
        display.join("\n")
    }

    pub fn rect(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                self.pixels[y][x] = true;
            }
        }
    }

    pub fn rotate_row(&mut self, row_index: usize, rotation: usize) {
        let row: Vec<bool> = self.pixels[row_index].iter().cloned().collect();
        let shifted: Vec<bool> = rotate(row, rotation);
        for (index, pixel) in shifted.iter().enumerate() {
            self.pixels[row_index][index] = *pixel;
        }
    }

    pub fn rotate_column(&mut self, column_index: usize, rotation: usize) {
        let mut column: Vec<bool> = vec![];
        for row in self.pixels.iter() {
            column.push( row[column_index]);
        }
        let shifted: Vec<bool> = rotate(column, rotation);
        for (index, pixel) in shifted.iter().enumerate() {
            self.pixels[index][column_index] = *pixel;
        }
    }

    pub fn parse(&mut self, raw_instruction: &str) {

        let rect_match = RECT.captures(raw_instruction);
        if rect_match.is_some() {
            let matches = rect_match.unwrap();
            let x = matches.at(1).unwrap().parse().unwrap();
            let y = matches.at(2).unwrap().parse().unwrap();
            self.rect(x, y);
            return;
        }

        let column_match = ROTATE_COLUMN.captures(raw_instruction);
        if column_match.is_some() {
            let matches = column_match.unwrap();
            let column = matches.at(1).unwrap().parse().unwrap();
            let rotation = matches.at(2).unwrap().parse().unwrap();
            self.rotate_column(column, rotation);
            return;
        }

        let row_match = ROTATE_ROW.captures(raw_instruction);
        if row_match.is_some() {
            let matches = row_match.unwrap();
            let row = matches.at(1).unwrap().parse().unwrap();
            let rotation = matches.at(2).unwrap().parse().unwrap();
            self.rotate_row(row, rotation);
            return;
        }
    }

}

fn rotate(mut row: Vec<bool>, rotation: usize) -> Vec<bool> {
    let len = row.len();
    let mut shifted: Vec<bool> = row.drain((len - rotation)..len).collect();
    shifted.append(&mut row);
    shifted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_display() {
        let _: Display = Display::new();
    }

    #[test]
    fn it_counts_on_pixels() {
        let display = Display::new();
        assert_eq!(0, display.on_pixels());
    }

    #[test]
    fn it_prints_display() {
        let output = "..................................................\n\
                      ..................................................\n\
                      ..................................................\n\
                      ..................................................\n\
                      ..................................................\n\
                      ..................................................";
        let display = Display::new();
        assert_eq!(output, display.print());
    }

    #[test]
    fn it_draws_rect() {
        let mut display = Display::new();
        display.rect(3, 2);

        let expected = "###...............................................\n\
                        ###...............................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_rotates_row() {
        let mut display = Display::new();
        display.rect(3, 2);
        display.rotate_row(0, 4);

        let expected = "....###...........................................\n\
                        ###...............................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_rotates_column() {
        let mut display = Display::new();
        display.rect(3, 2);
        display.rotate_column(1, 2);

        let expected = "#.#...............................................\n\
                        #.#...............................................\n\
                        .#................................................\n\
                        .#................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_rect_instruction() {
        let mut display = Display::new();
        display.parse("rect 3x2");

        let expected = "###...............................................\n\
                        ###...............................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_rotate_column() {
        let mut display = Display::new();
        display.rect(3,2);
        display.parse("rotate column x=1 by 1");

        let expected = "#.#...............................................\n\
                        ###...............................................\n\
                        .#................................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_rotates_row() {
        let mut display = Display::new();
        display.rect(3, 2);
        display.parse("rotate row y=0 by 4");

        let expected = "....###...........................................\n\
                        ###...............................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................\n\
                        ..................................................";
        println!("\nexpected:\n{}", expected);

        let actual = display.print();

        println!("actual:\n{}", actual);
        assert_eq!(expected, actual);
    }
}

