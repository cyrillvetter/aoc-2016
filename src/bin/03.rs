use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let possible_triangles = input
        .lines()
        .map(|l| l
            .split(" ")
            .map(|s| s.trim())
            .filter(|t| *t != "")
            .map(|f| f.parse::<u16>().unwrap())
            .sorted()
            .collect_vec())
        .filter(|t| t[0] + t[1] > t[2])
        .count();

    Solution::USize(possible_triangles)
}

// TODO: Refactor to closures only.
pub fn part_two(input: &str) -> Solution {
    const COLUMN_HEIGHT: usize = 3;

    let result = input
        .lines()
        .map(|l| l
            .split(" ")
            .map(|s| s.trim())
            .filter(|t| *t != "")
            .map(|f| f.parse::<u16>().unwrap())
            .collect_vec())
        .chunks(COLUMN_HEIGHT);

    let mut possible_triangles: usize = 0;

    for r in result.into_iter() {
        let chunk = r.collect_vec();
        for i in 0..COLUMN_HEIGHT {
            let mut column = vec![chunk[0][i], chunk[1][i], chunk[2][i]];
            column.sort();

            if column[0] + column[1] > column[2] {
                possible_triangles += 1;
            }
        }
    }

    Solution::USize(possible_triangles)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 3);
        assert_eq!(part_one(&input), Solution::USize(983));
        assert_eq!(part_two(&input), Solution::USize(1836));
    }
}
