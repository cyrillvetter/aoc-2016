use itertools::Itertools;
use regex::Regex;
use advent_of_code::Solution;

// TODO: Solve without regex.

pub fn part_one(input: &str) -> Solution {
    let brackets_regex = Regex::new("\\[(.*?)]").unwrap();
    let regex = Regex::new("(.*?)\\[(.*?)]|(.*)$").unwrap();

    let count = input
        .lines()
        .filter(|l|
            !brackets_regex.captures_iter(l).any(|c| has_abba(&c[1])) &&
                regex.captures_iter(l).any(|r| has_abba(r.get(1).unwrap_or_else(|| r.get(3).unwrap()).as_str())))
        .count();

    Solution::USize(count)
}

pub fn part_two(input: &str) -> Solution {
    let brackets_regex = Regex::new("\\[(.*?)]").unwrap();
    let regex = Regex::new("(.*?)\\[(.*?)]|(.*)$").unwrap();

    let count = input
        .lines()
        .filter(|l| {
            let abas = regex
                .captures_iter(l)
                .map(|r| r.get(1).unwrap_or_else(|| r.get(3).unwrap()).as_str())
                .map(|s| get_all_aba(s))
                .flatten()
                .collect_vec();

            let res = brackets_regex.captures_iter(l).any(|c| has_bab(&c[1], &abas));
            return res;
        })
        .count();

    Solution::USize(count)
}

fn has_abba(s: &str) -> bool {
    s.as_bytes().windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn get_all_aba(s: &str) -> Vec<&[u8]> {
    s.as_bytes().windows(3).filter(|w| w[0] == w[2] && w[0] != w[1]).collect()
}

fn has_bab(s: &str, aba: &Vec<&[u8]>) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2] && w[0] != w[1] && aba.iter().any(|i| w[0] == i[1] && w[1] == i[0]))
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 7);
        assert_eq!(part_one(&input), Solution::USize(110));
        assert_eq!(part_two(&input), Solution::USize(242));
    }
}
