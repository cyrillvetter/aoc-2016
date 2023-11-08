use advent_of_code::Solution;

const FIRST_HALF_BITMASK: u8 = 0b1111_0000;
const SECOND_HALF_BITMASK: u8 = 0b0000_1111;

pub fn part_one(_input: &str) -> Solution {
    const HASH_TIMES: u32 = 1;
    Solution::USize(solve(HASH_TIMES))
}

pub fn part_two(_input: &str) -> Solution {
    const HASH_TIMES: u32 = 2017;
    Solution::USize(solve(HASH_TIMES))
}

fn solve(hash_times: u32) -> usize {
    const SALT: &str = "yjdafjpo";

    let mut i: usize = 0;
    let mut found = 0;

    let mut hashes: Vec<[u8; 32]> = Vec::new();

    loop {
        let h = find_hash(SALT, i, hash_times, &mut hashes);

        if let Some(t) = first_triple(&h) {
            let mut j = i + 1;
            let limit = i + 1000;

            while j <= limit {
                if contains_quintuple(find_hash(SALT, j, hash_times, &mut hashes), t) {
                    found += 1;
                    break;
                }

                j += 1;
            }
        }

        if found == 64 {
            return i;
        }

        i += 1;
    }
}

fn find_hash(salt: &str, i: usize, hash_times: u32, hashes: &mut Vec<[u8; 32]>) -> [u8; 32] {
    if let Some(h) = hashes.get(i) {
        return *h;
    }

    let mut h = convert(md5::compute(salt.to_owned() + &i.to_string()).0);

    for _ in 0..hash_times-1 {
        h = convert(md5::compute(h).0);
    }

    hashes.push(h);
    h
}

fn convert(a: [u8; 16]) -> [u8; 32] {
    let mut n: [u8; 32] = [0; 32];

    for i in 0..16 {
        let j = i * 2;
        n[j] = to_ascii((a[i] & FIRST_HALF_BITMASK) >> 4);
        n[j + 1] = to_ascii(a[i] & SECOND_HALF_BITMASK);
    }

    n
}

fn to_ascii(c: u8) -> u8 {
    if c <= 9 {
        c + 48
    } else {
        c + 87
    }
}

fn first_triple(a: &[u8; 32]) -> Option<u8> {
    a.windows(3).filter(|w| w[0] == w[1] && w[1] == w[2]).map(|v| v[0]).next()
}

fn contains_quintuple(a: [u8; 32], v: u8) -> bool {
    a.windows(5).any(|w| v == w[0] && w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 14);
        assert_eq!(part_one(&input), Solution::USize(25427));
        assert_eq!(part_two(&input), Solution::USize(22045));
    }
}
