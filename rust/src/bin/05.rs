use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    const ZERO_COUNT: usize = 5;
    Solution::Str(compute_hash(input, ZERO_COUNT))
}

pub fn part_two(input: &str) -> Solution {
    const ZERO_COUNT: usize = 5;
    Solution::Str(compute_hash_2(input, ZERO_COUNT))
}

// TODO: Refactor these two methods.

fn compute_hash(input: &str, zero_count: usize) -> String {
    let half_len = zero_count / 2;
    let is_even = zero_count % 2 == 0;
    let compare_vec = vec![0; half_len];

    let mut password = String::new();

    let mut i = 0;

    for _ in 0..8 {
        loop {
            let hash = md5::compute(input.to_owned() + &i.to_string());
            let hash_parts = hash.0;

            if hash_parts[..half_len] == compare_vec {
                if !is_even {
                    if hash_parts[half_len] < 16 {
                        password.push(format!("{:x}", hash).as_bytes()[5] as char);
                        break;
                    }
                } else {
                    password.push(format!("{:x}", hash).as_bytes()[6] as char);
                    break;
                }
            }

            i += 1;
        }

        i += 1;
    }

    password
}

fn compute_hash_2(input: &str, zero_count: usize) -> String {
    let half_len = zero_count / 2;
    let is_even = zero_count % 2 == 0;
    let compare_vec = vec![0; half_len];

    let mut chars: [char; 8] = [ char::MAX, char::MAX, char::MAX, char::MAX, char::MAX, char::MAX, char::MAX, char::MAX ];
    let mut mutations = 0;

    let mut i = 0;

    while mutations < 8 {
        loop {
            let hash = md5::compute(input.to_owned() + &i.to_string());
            let hash_parts = hash.0;

            let mut found = false;

            if hash_parts[..half_len] == compare_vec {
                if !is_even {
                    if hash_parts[half_len] < 16 {
                        found = true;
                    }
                } else {
                    found = true;
                }
            }

            i += 1;

            if found {
                let hex = format!("{:x}", hash);
                let bytes = hex.as_bytes();
                let index = bytes[5] as usize - 48;

                if index >= 8 || chars[index] != char::MAX {
                    found = false;
                } else {
                    chars[index] = bytes[6] as char;
                    mutations += 1;
                    break;
                }
            }
        }
    }

    chars.iter().collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 5);
        assert_eq!(part_one(&input), Solution::Str(String::from("4543c154")));
        assert_eq!(part_two(&input), Solution::Str(String::from("1050cbbd")));
    }
}
