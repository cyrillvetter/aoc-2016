use advent_of_code::Solution;

const FIRST_HALF_BITMASK: u8 = 0b1111_0000;
const SECOND_HALF_BITMASK: u8 = 0b0000_1111;

const ADJACENT: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const DIRECTIONS: [&str; 4] = ["U", "D", "L", "R"];

pub fn part_one(_input: &str) -> Solution {
    const PASSCODE: &str = "njfxhljp";

    let mut stack: Vec<(String, (i8, i8))> = Vec::new();
    stack.push((String::new(), (0, 0)));

    let mut shortest_path: Option<String> = None;

    while let Some((s, (x, y))) = stack.pop() {
        if x == 3 && y == 3 {
            match &shortest_path {
                None => { shortest_path = Some(s.clone()) }
                Some(shortest) => {
                    if s.len() < shortest.len() {
                        shortest_path = Some(s.clone());
                    }
                }
            }

            continue;
        }

        if shortest_path.is_some() && shortest_path.as_ref().unwrap().len() <= s.len() {
            continue;
        }

        let h = hash(&(PASSCODE.to_owned() + &s));
        for (i, (ad_x, ad_y)) in ADJACENT.iter().enumerate() {
            let next = (x + *ad_x, y + *ad_y);
            if is_within_bound(next) && h[i] {
                stack.push((s.clone() + &DIRECTIONS[i], next));
            }
        }
    }

    Solution::Str(shortest_path.unwrap())
}

pub fn part_two(_input: &str) -> Solution {
    const PASSCODE: &str = "njfxhljp";

    let mut stack: Vec<(String, (i8, i8))> = Vec::new();
    stack.push((String::new(), (0, 0)));

    let mut longest_path = 0;

    while let Some((s, (x, y))) = stack.pop() {
        if x == 3 && y == 3 {
            longest_path = longest_path.max(s.len());
            continue;
        }

        let h = hash(&(PASSCODE.to_owned() + &s));
        for (i, (ad_x, ad_y)) in ADJACENT.iter().enumerate() {
            let next = (x + *ad_x, y + *ad_y);
            if is_within_bound(next) && h[i] {
                stack.push((s.clone() + &DIRECTIONS[i], next));
            }
        }
    }

    Solution::USize(longest_path)
}

fn hash(data: &String) -> [bool; 4] {
    let h = md5::compute(data).0;

    [
        is_open((h[0] & FIRST_HALF_BITMASK) >> 4),
        is_open(h[0] & SECOND_HALF_BITMASK),
        is_open((h[1] & FIRST_HALF_BITMASK) >> 4),
        is_open(h[1] & SECOND_HALF_BITMASK)
    ]
}

fn is_open(c: u8) -> bool {
    match to_ascii(c) {
        b'b' | b'c' | b'd' | b'e' | b'f' => true,
        _ => false
    }
}

fn to_ascii(c: u8) -> u8 {
    if c <= 9 { c + 48 } else { c + 87 }
}

fn is_within_bound(pos: (i8, i8)) -> bool {
    pos.0 >= 0 && pos.0 <= 3 && pos.1 >= 0 && pos.1 <= 3
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 17);
        assert_eq!(part_one(&input), Solution::Str(String::from("DURLDRRDRD")));
        assert_eq!(part_two(&input), Solution::USize(650));
    }
}
