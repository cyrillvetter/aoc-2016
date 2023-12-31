use std::collections::HashSet;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let mut loc: (i32, i32) = (0, 0);
    let mut degrees = 0;

    for d in input.split(", ") {
        let (direction, steps) = d.split_at(1);
        let steps = steps.parse::<i32>().unwrap();

        degrees = match direction {
            "R" => calculate_degrees(degrees, 90),
            "L" => calculate_degrees(degrees, -90),
            _ => unreachable!(),
        };

        loc = match degrees {
            0 => (loc.0 + steps, loc.1),
            90 => (loc.0, loc.1 + steps),
            180 => (loc.0 - steps, loc.1),
            270 => (loc.0, loc.1 - steps),
            _ => unreachable!(),
        }
    }

    Solution::I32(loc.0.abs() + loc.1.abs())
}

pub fn part_two(input: &str) -> Solution {
    let mut loc: (i32, i32) = (0, 0);
    let mut degrees = 0;
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    visits.insert(loc);

    for d in input.split(", ") {
        let (direction, steps) = d.split_at(1);
        let steps = steps.parse::<i32>().unwrap();

        degrees = match direction {
            "R" => calculate_degrees(degrees, 90),
            "L" => calculate_degrees(degrees, -90),
            _ => unreachable!(),
        };

        let mut temp_loc = loc;

        for l in 1..=steps {
            temp_loc = match degrees {
                0 => (loc.0 + l, loc.1),
                90 => (loc.0, loc.1 + l),
                180 => (loc.0 - l, loc.1),
                270 => (loc.0, loc.1 - l),
                _ => unreachable!(),
            };

            if visits.contains(&temp_loc) {
                return Solution::I32(temp_loc.0.abs() + temp_loc.1.abs());
            }

            visits.insert(temp_loc);
        }

        loc = temp_loc;
    }

    unreachable!()
}

fn calculate_degrees(curr: i32, next: i32) -> i32 {
    let n = curr + next;
    if n == -90 {
        return 270;
    } else if n == 360 {
        return 0;
    }

    n
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_one(&input), Solution::I32(273));
        assert_eq!(part_two(&input), Solution::I32(115));
    }
}
