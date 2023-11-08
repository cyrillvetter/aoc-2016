use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let l = input.lines().map(|l| l.as_bytes()).collect_vec();
    let word_len = l[0].len();

    let message: String = (0..word_len)
        .into_iter()
        .map(|n| l
            .iter()
            .map(|c| c[n])
            .sorted_unstable()
            .group_by(|c| *c)
            .into_iter()
            .map(|(k, g)| (k, g.count()))
            .max_by_key(|&(_, count)| count)
            .unwrap().0 as char)
        .collect();

    Solution::Str(message)
}

pub fn part_two(input: &str) -> Solution {
    let l = input.lines().map(|l| l.as_bytes()).collect_vec();
    let word_len = l[0].len();

    let message: String = (0..word_len)
        .into_iter()
        .map(|n| l
            .iter()
            .map(|c| c[n])
            .sorted_unstable()
            .group_by(|c| *c)
            .into_iter()
            .map(|(k, g)| (k, g.count()))
            .min_by_key(|&(_, count)| count)
            .unwrap().0 as char)
        .collect();

    Solution::Str(message)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 6);
        assert_eq!(part_one(&input), Solution::Str(String::from("qoclwvah")));
        assert_eq!(part_two(&input), Solution::Str(String::from("ryrgviuv")));
    }
}
