use std::collections::{HashSet, VecDeque};
use advent_of_code::Solution;

const FAVORITE_NUMBER: i32 = 1362;
const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part_one(_input: &str) -> Solution {
    const START_POINT: (i32, i32) = (1, 1);
    const END_POS: (i32, i32) = (31, 39);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();

    queue.push_back((START_POINT.clone(), 1));
    visited.insert(START_POINT);

    while let Some(((x, y), c)) = queue.pop_front() {
        if x == END_POS.0 && y == END_POS.1 {
            return Solution::I32(c - 1);
        }

        for (ax, ay) in &ADJACENT {
            let next = (x + *ax, y + *ay);
            if is_open_space(next) && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, c + 1));
            }
        }
    }

    unreachable!();
}

pub fn part_two(_input: &str) -> Solution {
    const START_POINT: (i32, i32) = (1, 1);
    const STEPS_LIMIT: i32 = 50;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();

    queue.push_back((START_POINT.clone(), 1));
    visited.insert(START_POINT);

    while let Some(((x, y), c)) = queue.pop_front() {
        if c <= STEPS_LIMIT {
            for (ax, ay) in &ADJACENT {
                let next = (x + *ax, y + *ay);
                if is_open_space(next) && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, c + 1));
                }
            }
        }
    }

    Solution::USize(visited.len())
}

fn is_open_space(p: (i32, i32)) -> bool {
    if p.0.is_negative() || p.1.is_negative() {
        return false;
    }

    (p.0 * p.0 + 3 * p.0 + 2 * p.0 * p.1 + p.1 + p.1 * p.1 + FAVORITE_NUMBER).count_ones() % 2 == 0
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 13);
        assert_eq!(part_one(&input), Solution::I32(82));
        assert_eq!(part_two(&input), Solution::USize(138));
    }
}
