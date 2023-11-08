use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let time = solve(parse(input));
    Solution::USize(time)
}

pub fn part_two(input: &str) -> Solution {
    let mut discs = parse(input);
    discs.push((11, 0));

    let time = solve(discs);
    Solution::USize(time)
}

fn solve(discs: Vec<(usize, usize)>) -> usize {
    (0..)
        .filter(|time| discs
            .iter()
            .enumerate()
            .all(|(i, (total, pos))| (pos + i + 1 + time) % total == 0))
        .next().unwrap()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| l.trim_end_matches('.').split(" ").collect_vec())
        .map(|s| (s[3].parse::<usize>().unwrap(), s[11].parse::<usize>().unwrap()))
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 15);
        assert_eq!(part_one(&input), Solution::USize(122318));
        assert_eq!(part_two(&input), Solution::USize(3208583));
    }
}
