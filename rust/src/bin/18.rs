use advent_of_code::Solution;

const ROW_LEN: usize = 100;

pub fn part_one(input: &str) -> Solution {
    const ROW_AMOUNT: usize = 40;
    Solution::USize(count_safe_tiles(input, ROW_AMOUNT))
}

pub fn part_two(input: &str) -> Solution {
    const ROW_AMOUNT: usize = 400000;
    Solution::USize(count_safe_tiles(input, ROW_AMOUNT))
}

// TODO: Replace array with 128-bit integer.

fn count_safe_tiles(input: &str, total_rows: usize) -> usize {
    let mut prev: [bool; ROW_LEN + 2] = [true; ROW_LEN + 2];
    let mut curr = prev;

    let mut safe_count: usize = 0;

    for (i, b) in input.as_bytes().iter().enumerate() {
        let val = *b == b'.';
        prev[i + 1] = val;

        if val {
            safe_count += 1;
        }
    }

    for _ in 0..total_rows-1 {
        for i in 1..ROW_LEN+1 {
            let left = prev[i - 1];
            let center = prev[i];
            let right = prev[i + 1];

            let val = !((!left && !center && right) ||
                (left && !center && !right) ||
                (!left && center && right) ||
                (left && center && !right));

            curr[i] = val;

            if val {
                safe_count += 1;
            }
        }

        prev = curr;
    }

    safe_count
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 18);
        assert_eq!(part_one(&input), Solution::USize(1956));
        assert_eq!(part_two(&input), Solution::USize(19995121));
    }
}
