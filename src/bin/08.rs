use itertools::Itertools;
use advent_of_code::Solution;

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 50;

pub fn part_one(input: &str) -> Solution {
    let screen = get_code_screen(input);
    let lit_pixels = screen.iter().map(|p| p.iter().filter(|v| **v).count()).sum();
    Solution::USize(lit_pixels)
}

pub fn part_two(input: &str) -> Solution {
    let screen = get_code_screen(input);
    let mut s = screen.iter().map(|r| r.iter().map(|v| if *v { "#" } else { "." }).collect::<String>()).join("\n");
    s.insert(0, '\n');
    Solution::Str(s)
}

fn get_code_screen(input: &str) -> [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    let mut screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for l in input.lines() {
        if l.starts_with("rect") {
            let parts = l.split(" ").nth(1).unwrap().split("x").map(|k| k.parse::<usize>().unwrap()).collect_tuple::<(_, _)>().unwrap();
            for x in 0..parts.1 {
                for y in 0..parts.0 {
                    screen[x][y] = true;
                }
            }
        } else {
            let parts = l.split(" ").collect_vec();

            let col_row = parts[2].split("=").nth(1).unwrap().parse::<usize>().unwrap();
            let pixels = parts[4].parse::<usize>().unwrap();

            if parts[1].eq("column") {
                let mut col_copy = (0..SCREEN_HEIGHT).into_iter().map(|i| screen[i][col_row]).collect_vec();

                for (i, v) in col_copy.drain(..).enumerate().map(|(i, v)| ((i + pixels) % SCREEN_HEIGHT, v)) {
                    screen[i][col_row] = v;
                }
            } else {
                for (i, v) in screen[col_row].clone().iter().enumerate().map(|(i, v)| ((i + pixels) % SCREEN_WIDTH, v)) {
                    screen[col_row][i] = *v;
                }
            }
        }
    }

    screen
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 8);
        assert_eq!(part_one(&input), Solution::USize(110));
        let expected_part2 = "\n\
        ####...##.#..#.###..#..#..##..###..#....#...#..##.\n\
        ...#....#.#..#.#..#.#.#..#..#.#..#.#....#...#...#.\n\
        ..#.....#.####.#..#.##...#....#..#.#.....#.#....#.\n\
        .#......#.#..#.###..#.#..#....###..#......#.....#.\n\
        #....#..#.#..#.#.#..#.#..#..#.#....#......#..#..#.\n\
        ####..##..#..#.#..#.#..#..##..#....####...#...##..";
        assert_eq!(part_two(&input), Solution::Str(String::from(expected_part2)));
    }
}
