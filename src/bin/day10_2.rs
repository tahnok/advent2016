use std::collections::HashMap;
use std::cmp;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut bots = HashMap::new();
    let mut instructions = vec![];
    parse(&input, &mut bots, &mut instructions);

    let outputs = work(&mut bots, instructions);
    let result = outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap();
    println!("{}", result);
}

pub struct Bot {
    number: usize,
    low: Give,
    high: Give,
    chips: Option<usize>,
}

impl Bot {
    fn take(&mut self, chip: usize, mut bots: &mut HashMap<usize, Bot>, mut outputs: &mut HashMap<usize, usize>) {
        if self.chips == None {
            self.chips = Some(chip);
        } else {
            let min = cmp::min(self.chips.unwrap(), chip);
            let max = cmp::max(self.chips.unwrap(), chip);

            match self.low {
                Give::Bot(i) => {
                    let mut low_bot = bots.remove(&i).unwrap();
                    low_bot.take(min, &mut bots, &mut outputs);
                    bots.insert(low_bot.number, low_bot);
                },
                Give::Output(i) => { outputs.insert(i, min); },
            }


            match self.high {
                Give::Bot(i) => {
                    let mut high_bot = bots.remove(&i).unwrap();
                    high_bot.take(max, &mut bots, &mut outputs);
                    bots.insert(high_bot.number, high_bot);
                },
                Give::Output(i) => { outputs.insert(i, max); },
            }
            self.chips = None;
        }
    }
}

impl FromStr for Bot {
    type Err = BotErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let number = parts.nth(1).unwrap().parse().unwrap();
        let give_low = parts.nth(3).unwrap();
        let low = match give_low {
            "bot" => Give::Bot(parts.next().unwrap().parse().unwrap()),
            "output" => Give::Output(parts.next().unwrap().parse().unwrap()),
            _ => panic!("don't know what to do"),
        };
        let give_high = parts.nth(3).unwrap();
        let high = match give_high {
            "bot" => Give::Bot(parts.next().unwrap().parse().unwrap()),
            "output" => Give::Output(parts.next().unwrap().parse().unwrap()),
            _ => panic!("don't know what to do"),
        };
        Ok(Bot{
            number: number,
            low: low,
            high: high,
            chips: None,
        })
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Give {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
pub struct BotErr {
}

pub fn parse(input: &str, bots: &mut HashMap<usize, Bot>, instructions: &mut Vec<(usize, usize)>) {
    for mut line in input.lines() {
        line = line.trim();
        if line.starts_with("bot") {
            let bot: Bot = line.parse().unwrap();
            bots.insert(bot.number, bot);
        } else {
            let mut parts = line.split_whitespace();
            let value = parts.nth(1).unwrap().parse().unwrap();
            let bot = parts.nth(3).unwrap().parse().unwrap();
            instructions.push((bot, value));
        }
    }
}

pub fn work(mut bots: &mut HashMap<usize, Bot>, instructions: Vec<(usize, usize)>) -> HashMap<usize, usize> {
    let mut outputs = HashMap::new();
    for (bot_number, chip) in instructions {
        let mut bot = bots.remove(&bot_number).unwrap();
        bot.take(chip, &mut bots, &mut outputs);
        bots.insert(bot.number, bot);
    }

    outputs
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn it_can_make_a_bot_with_a_number() {
        let bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        assert_eq!(2, bot.number);
    }

    #[test]
    fn it_can_make_a_bot_with_a_low_to_bot() {
        let bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        assert_eq!(Give::Bot(1), bot.low);
    }

    #[test]
    fn it_can_make_a_bot_with_a_hight_to_bot() {
        let bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        assert_eq!(Give::Bot(0), bot.high);
    }

    #[test]
    fn it_can_make_a_bot_with_low_to_output() {
        let bot: Bot = "bot 0 gives low to output 2 and high to output 0".parse().unwrap();
        assert_eq!(Give::Output(2), bot.low);
    }

    #[test]
    fn it_can_make_a_bot_with_high_to_output() {
        let bot: Bot = "bot 0 gives low to output 2 and high to output 0".parse().unwrap();
        assert_eq!(Give::Output(0), bot.high);
    }

    #[test]
    fn it_takes_one_chip() {
        let mut outputs = HashMap::new();
        let mut bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        bot.take(4, &mut get_bots(), &mut outputs);
        assert_eq!(Some(4), bot.chips);
    }

    fn get_bots() -> HashMap<usize, Bot> {
        let mut bots = HashMap::new();
        let bot0: Bot = "bot 0 gives low to output 2 and high to output 0".parse().unwrap();
        let bot1: Bot = "bot 1 gives low to output 1 and high to bot 0".parse().unwrap();
        bots.insert(bot0.number, bot0);
        bots.insert(bot1.number, bot1);
        bots
    }

    #[test]
    fn it_takes_chips() {
        let mut bots = get_bots();
        let mut outputs = HashMap::new();
        let mut bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        bot.take(4, &mut bots, &mut outputs);
        bot.take(2, &mut bots, &mut outputs);
        assert_eq!(bot.chips, None);
    }

    #[test]
    fn it_takes_2_chips_and_gives_to_low_bot() {
        let mut outputs = HashMap::new();
        let mut bots = get_bots();
        let mut bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        bot.take(4, &mut bots, &mut outputs);
        bot.take(2, &mut bots, &mut outputs);

        assert_eq!(Some(2), bots.get(&1).unwrap().chips);
    }

    #[test]
    fn it_takes_2_chips_and_gives_to_high_bot() {
        let mut outputs = HashMap::new();
        let mut bots = get_bots();
        let mut bot: Bot = "bot 2 gives low to bot 1 and high to bot 0".parse().unwrap();
        bot.take(4, &mut bots, &mut outputs);
        bot.take(2, &mut bots, &mut outputs);

        assert_eq!(Some(4), bots.get(&0).unwrap().chips);
    }

    #[test]
    fn it_takes_2_chips_and_outputs_buckets() {
        let mut bots = HashMap::new();
        let mut outputs = HashMap::new();
        let mut bot: Bot = "bot 0 gives low to output 2 and high to output 0".parse().unwrap();
        bot.take(4, &mut bots, &mut outputs);
        bot.take(2, &mut bots, &mut outputs);
        assert_eq!(outputs.get(&2).unwrap(), &2);
        assert_eq!(outputs.get(&0).unwrap(), &4);
    }

    #[test]
    fn it_builds_bot_list() {
        let input = "value 5 goes to bot 2
            bot 2 gives low to bot 1 and high to bot 0
            value 3 goes to bot 1
            bot 1 gives low to output 1 and high to bot 0
            bot 0 gives low to output 2 and high to output 0
            value 2 goes to bot 2";

        let mut bots = HashMap::new();
        let mut instructions = vec![];
        parse(input, &mut bots, &mut instructions);
        assert_eq!(bots.get(&0).unwrap().number, 0);
        assert_eq!(bots.get(&1).unwrap().number, 1);
        assert_eq!(bots.get(&2).unwrap().number, 2);
    }

    #[test]
    fn it_builds_instruction_list() {
        let input = "value 5 goes to bot 2
            bot 2 gives low to bot 1 and high to bot 0
            value 3 goes to bot 1
            bot 1 gives low to output 1 and high to bot 0
            bot 0 gives low to output 2 and high to output 0
            value 2 goes to bot 2";

        let mut bots = HashMap::new();
        let mut instructions = vec![];
        parse(input, &mut bots, &mut instructions);
        let output = vec![
            (2, 5),
            (1, 3),
            (2, 2),
        ];
        assert_eq!(output, instructions);
    }

    #[test]
    fn it_works() {
        let input = "value 5 goes to bot 2
            bot 2 gives low to bot 1 and high to bot 0
            value 3 goes to bot 1
            bot 1 gives low to output 1 and high to bot 0
            bot 0 gives low to output 2 and high to output 0
            value 2 goes to bot 2";

        let mut bots = HashMap::new();
        let mut instructions = vec![];
        parse(input, &mut bots, &mut instructions);

        let outputs = work(&mut bots, instructions);
        assert_eq!(outputs.get(&0).unwrap(), &5);
        assert_eq!(outputs.get(&1).unwrap(), &2);
        assert_eq!(outputs.get(&2).unwrap(), &3);
    }

}
