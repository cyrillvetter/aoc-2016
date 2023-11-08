use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let len = input.len();
    let mut i: usize = 0;

    let mut decompressed_len: usize = 0;

    while i < len {
        match &input[i..len].split_once('(') {
            Some(first_split) => {
                let second_split = first_split.1.split_once(')').unwrap();
                let values = second_split.0.split_once('x').unwrap();

                let next = values.0.parse::<usize>().unwrap();
                let times = values.1.parse::<usize>().unwrap();

                i += first_split.0.len() + 3 + values.0.len() + values.1.len() + next;
                decompressed_len += first_split.0.len() + (next * times);
            }
            None => {
                decompressed_len += len - i;
                break;
            }
        }
    }

    Solution::USize(decompressed_len)
}

pub fn part_two(input: &str) -> Solution {
    let decompressed_len = version2_decompressed_len(input);
    Solution::USize(decompressed_len)
}

fn version2_decompressed_len(input: &str) -> usize {
    let len = input.len();
    let mut i: usize = 0;

    let mut decompressed_len: usize = 0;

    while i < len {
        match &input[i..len].split_once('(') {
            Some(first_split) => {
                let second_split = first_split.1.split_once(')').unwrap();
                let values = second_split.0.split_once('x').unwrap();

                let next = values.0.parse::<usize>().unwrap();
                let times = values.1.parse::<usize>().unwrap();

                i += first_split.0.len() + 3 + values.0.len() + values.1.len();
                let inner = version2_decompressed_len(&input[i..i+next]);

                i += next;
                decompressed_len += first_split.0.len() + (inner * times);
            }
            None => {
                decompressed_len += len - i;
                break;
            }
        }
    }

    decompressed_len
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 9);
        assert_eq!(part_one(&input), Solution::USize(107035));
        assert_eq!(part_two(&input), Solution::USize(11451628995));
    }
}
