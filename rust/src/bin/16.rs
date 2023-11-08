use advent_of_code::Solution;

pub fn part_one(_input: &str) -> Solution {
    const INPUT: &str = "01111001100111011";
    const LEN: usize = 272;

    Solution::Str(checksum(INPUT, LEN))
}

pub fn part_two(_input: &str) -> Solution {
    const INPUT: &str = "01111001100111011";
    const LEN: usize = 35_651_584;

    Solution::Str(checksum(INPUT, LEN))
}

fn checksum(input: &str, len: usize) -> String {
    let mut curr_len: usize = input.len();
    let mut a = String::new();

    for b in input.as_bytes().iter() {
        a.push(*b as char);
    }

    while curr_len < len {
        let next_len = curr_len.min(len);
        a.push('0');
        for i in 0..next_len.min(len - next_len - 1) {
            let n = a.as_bytes()[next_len - i - 1];
            a.push(if n == b'0' { '1' } else { '0' });
        }

        curr_len = curr_len * 2 + 1;
    }

    a
        .as_bytes()
        .chunks(2_usize.pow(len.trailing_zeros()))
        .map(|c| if c.iter().filter(|v| **v == b'1').count() % 2 == 0 { '1' } else { '0' })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 16);
        assert_eq!(part_one(&input), Solution::Str(String::from("11111000111110000")));
        assert_eq!(part_two(&input), Solution::Str(String::from("10111100110110100")));
    }
}
