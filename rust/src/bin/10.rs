use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    const LOW_MICROCHIP: u32 = 17;
    const HIGH_MICROCHIP: u32 = 61;
    let mut bots = parse(input);

    loop {
        let (bot_id, bot) = get_first_full_bot(&bots);

        if bot.values[0] == LOW_MICROCHIP && bot.values[1] == HIGH_MICROCHIP {
            return Solution::U32(bot_id);
        }

        if let Next::Bot(id) = bot.low {
            bots.get_mut(&id).unwrap().insert_value(bot.values[0]);
        }

        if let Next::Bot(id) = bot.high {
            bots.get_mut(&id).unwrap().insert_value(bot.values[1]);
        }

        bots.get_mut(&bot_id).unwrap().values.clear();
    }
}

pub fn part_two(input: &str) -> Solution {
    let mut bots = parse(input);
    let mut output_values: Vec<u32> = Vec::new(); // The values of output 0, 1 and 2.

    loop {
        if output_values.len() == 3 {
            return Solution::U32(output_values.iter().product());
        }

        let (bot_id, bot) = get_first_full_bot(&bots);

        if let Next::Bot(id) = bot.low {
            bots.get_mut(&id).unwrap().insert_value(bot.values[0]);
        } else if let Next::Output(id) = bot.low {
            if id >= 0 && id <= 2 { output_values.push(bot.values[0]) }
        }

        if let Next::Bot(id) = bot.high {
            bots.get_mut(&id).unwrap().insert_value(bot.values[1]);
        } else if let Next::Output(id) = bot.high {
            if id >= 0 && id <= 2 { output_values.push(bot.values[1]) }
        }

        bots.get_mut(&bot_id).unwrap().values.clear();
    }
}

#[derive(Clone)]
struct Bot {
    low: Next,
    high: Next,
    values: Vec<u32>,
}

impl Bot {
    fn insert_value(&mut self, value: u32) {
        self.values.push(value);
        self.values.sort();
    }
}

#[derive(Clone)]
enum Next {
    Bot(u32),
    Output(u32)
}

fn get_first_full_bot(bots: &HashMap<u32, Bot>) -> (u32, Bot) {
    bots.iter().filter(|(_, b)| b.values.len() == 2).map(|(id, b)| (*id, b.clone())).next().unwrap()
}

fn parse(input: &str) -> HashMap<u32, Bot> {
    let mut bots: HashMap<u32, Bot> = HashMap::new();

    for l in input.lines().filter(|l| l.starts_with("bot")) {
        let split = l.split(" ").collect_vec();
        let bot_id = split[1].parse::<u32>().unwrap();
        let low_id = split[6].parse::<u32>().unwrap();
        let high_id = split[11].parse::<u32>().unwrap();

        let bot = Bot {
            low: if split[5].eq("bot") { Next::Bot(low_id) } else { Next::Output(low_id) },
            high: if split[10].eq("bot") { Next::Bot(high_id) } else { Next::Output(high_id) },
            values: Vec::with_capacity(2)
        };

        bots.insert(bot_id, bot);
    }

    for l in input.lines().filter(|l| l.starts_with("value")) {
        let split = l.split(" ").collect_vec();
        let value = split[1].parse::<u32>().unwrap();
        let bot_id = split[5].parse::<u32>().unwrap();

        bots.get_mut(&bot_id).unwrap().insert_value(value);
    }

    bots
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 10);
        assert_eq!(part_one(&input), Solution::U32(181));
        assert_eq!(part_two(&input), Solution::U32(12567));
    }
}
