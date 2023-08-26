use itertools::Itertools;
use advent_of_code::Solution;
use crate::Instruction::{Cpy, Dec, Inc, Jnz};
use crate::Loc::{Register, Value};

pub fn part_one(input: &str) -> Solution {
    let register_a = calculate_register_a(input, [0; 4]);
    Solution::I32(register_a)
}

pub fn part_two(input: &str) -> Solution {
    let register_a = calculate_register_a(input, [0, 0, 1, 0]);
    Solution::I32(register_a)
}

fn calculate_register_a(input: &str, mut registers: [i32; 4]) -> i32 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| Instruction::from(l))
        .collect();
    let total_inst = instructions.len();

    let mut inst: usize = 0;

    while inst < total_inst {
        let next_inst = &instructions[inst];
        match next_inst {
            Cpy(l, r) => {
                match l {
                    Value(v) => registers[*r] = *v,
                    Register(n) => registers[*r] = registers[*n]
                }
            }
            Inc(r) => registers[*r] += 1,
            Dec(r) => registers[*r] -= 1,
            Jnz(l, j) => {
                let v = match l {
                    Value(v) => *v,
                    Register(r) => registers[*r]
                };

                if v != 0 {
                    inst = (inst as isize + *j) as usize - 1;
                }
            }
        }

        inst += 1;
    }

    registers[0]
}

fn convert_register(r: &str) -> usize {
    match r {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => unreachable!()
    }
}

enum Instruction {
    Cpy(Loc, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Loc, isize)
}

impl Instruction {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect_vec();
        match split[0] {
            "cpy" => {
                Cpy(Loc::from(split[1]), convert_register(split[2]))
            },
            "inc" => Inc(convert_register(split[1])),
            "dec" => Dec(convert_register(split[1])),
            "jnz" => Jnz(Loc::from(split[1]), split[2].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

enum Loc {
    Value(i32),
    Register(usize)
}

impl Loc {
    fn from(s: &str) -> Self {
        match s.parse::<i32>() {
            Ok(r) => Value(r),
            Err(_) => Register(convert_register(s))
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 12);
        assert_eq!(part_one(&input), Solution::I32(318020));
        assert_eq!(part_two(&input), Solution::I32(9227674));
    }
}
