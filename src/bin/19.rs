use advent_of_code::Solution;

// The Josephus Problem (see: https://www.youtube.com/watch?v=uCsD3ZGzMgE).
pub fn part_one(_input: &str) -> Solution {
    const INPUT: u32 = 3005290;
    let result = (INPUT - (1 << (u32::BITS - INPUT.leading_zeros() - 1))) * 2 + 1;
    Solution::U32(result)
}

pub fn part_two(_input: &str) -> Solution {
    const INPUT: usize = 3005290;
    Solution::Empty
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 19);
        assert_eq!(part_one(&input), Solution::U32(1816277));
        assert_eq!(part_two(&input), Solution::Empty);
    }
}
