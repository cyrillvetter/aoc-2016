use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let section_id_sum = input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(name, _, checksum)| name
            .chars()
            .filter(|c| *c != '-')
            .sorted_unstable()
            .group_by(|k| *k)
            .into_iter()
            .map(|(k, g)| (k, g.count()))
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .take(5)
            .all(|(c, _)| checksum.contains(c)))
        .map(|(_, section_id, _)| section_id)
        .sum();

    Solution::U32(section_id_sum)
}

pub fn part_two(input: &str) -> Solution {
    const ALPHABET_LEN: u8 = 26;
    const STORAGE_ROOM_NAME: &str = "northpole object storage";

    let storage_room_section_id = input
        .lines()
        .map(|l| parse_line(l))
        .map(|(c, section_id, k)| {
            let decrypted_room_name: String = c
                .chars()
                .map(|n| {
                    if n == '-' { return ' ' }

                    ((((n as u8 - 97) + ((section_id % ALPHABET_LEN as u32) as u8)) % ALPHABET_LEN) + 97) as char
                })
                .collect();

            (decrypted_room_name, section_id)
        })
        .find(|r| r.0.eq(STORAGE_ROOM_NAME)).unwrap().1;

    Solution::U32(storage_room_section_id)
}

fn parse_line(s: &str) -> (&str, u32, &str) {
    let (encrypted_name, remaining) = s.rsplit_once("-").unwrap();

    let (section_id_part, checksum_part) = remaining.split_once("[").unwrap();
    let section_id = section_id_part.parse::<u32>().unwrap();
    let checksum = &checksum_part[0..checksum_part.len() - 1];

    (encrypted_name, section_id, checksum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 4);
        assert_eq!(part_one(&input), Solution::U32(158835));
        assert_eq!(part_two(&input), Solution::U32(993));
    }
}
