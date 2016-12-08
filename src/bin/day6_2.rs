use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::Lines;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let message = rebuild_message(input.lines());
    println!("{}", message);
}


pub fn count_letters(lines: Lines) -> [HashMap<char, u32>; 8] {
    let mut output = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    for line in lines {
        for (i, character) in line.trim().chars().enumerate() {
            let count = output[i].entry(character).or_insert(0);
            *count += 1;
        }
    }

    output
}

pub fn min_entry(map: &HashMap<char, u32>) -> char {
    *map.iter().min_by_key(|x| x.1).unwrap().0
}

pub fn rebuild_message(lines: Lines) -> String {
    count_letters(lines)
        .iter()
        .map(|count| min_entry(count))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn it_counts_letter_frequency_per_position() {
        let input = "eedadnza\nervteeaz";

        let mut first = HashMap::new();
        first.insert('e', 2);

        let mut second = HashMap::new();
        second.insert('e', 1);
        second.insert('r', 1);

        let mut third = HashMap::new();
        third.insert('d', 1);
        third.insert('v', 1);

        let mut fourth = HashMap::new();
        fourth.insert('a', 1);
        fourth.insert('t', 1);

        let mut fifth = HashMap::new();
        fifth.insert('d', 1);
        fifth.insert('e', 1);

        let mut sixth = HashMap::new();
        sixth.insert('n', 1);
        sixth.insert('e', 1);

        let mut seventh = HashMap::new();
        seventh.insert('z', 1);
        seventh.insert('a', 1);

        let mut eigth = HashMap::new();
        eigth.insert('a', 1);
        eigth.insert('z', 1);

        let output = [
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
            seventh,
            eigth,
        ];

        assert_eq!(output, count_letters(input.lines()));
    }

    #[test]
    fn it_finds_min_entry() {
        let mut sixth = HashMap::new();
        sixth.insert('d', 3);
        sixth.insert('n', 2);
        sixth.insert('e', 1);

        assert_eq!('e', min_entry(&sixth));
    }

    #[test]
    fn it_rebuilds_message() {
        let input = "eedadnzz\ndrvteezz\neandsrzz\nraavrdzz\natevrszz\ntsrnevzz\nsdttsazz\nrasrtvzz\n\
            nssdtszz\nntnadazz\nsvetvezz\ntesnvtzz\nvntsndzz\nvrdearzz\ndvrsenzz\nenararzz";
        assert_eq!("adventzz", rebuild_message(input.lines()));
    }
}
