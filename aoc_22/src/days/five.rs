use super::utility;
use crate::problem::Problem;

use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct Ship {
    ship: Vec<VecDeque<char>>,
    instructions: Vec<Move>,
}
impl Ship {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

pub struct Five {}

impl Problem for Five {
    fn part_one(&self, input: &str) -> String {
        let mut aoc = Ship::new();

        let content = utility::input_to_vec_string(input.to_string());

        for lines in content {
            if lines.contains('[') {
                for pair in lines
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
                {
                    let stack = (pair.0 - 1) / 4;
                    //println!("pair: {pair:?} , stack: {stack}");
                    while aoc.ship.len() < stack + 1 {
                        aoc.ship.push(VecDeque::new());
                    }
                    aoc.ship[stack].push_front(pair.1)
                }
            }

            if lines.contains("move") {
                let words = lines.split_whitespace().collect::<Vec<&str>>();
                //println!("{:?}", words);
                if lines.contains("move") {
                    let amount: usize = words[1].parse().unwrap();
                    let from: usize = words[3].parse::<usize>().unwrap() - 1;
                    let to: usize = words[5].parse::<usize>().unwrap() - 1;
                    aoc.instructions.push(Move { amount, from, to });
                }
            }
        }
        //println!("{:?}", aoc);

        let mut ship1 = aoc.ship.clone();

        // move crates from stack to stack
        for i in &aoc.instructions {
            for _ in 0..i.amount {
                let temp = ship1[i.from].pop_back().unwrap();
                ship1[i.to].push_back(temp);
            }
        }
        let mut answer = "".to_string();
        for stack in aoc.ship {
            answer.push(*stack.back().unwrap());
        }

        answer
    }
    fn part_two(&self, input: &str) -> String {
        let mut aoc = Ship::new();

        let content = utility::input_to_vec_string(input.to_string());

        for lines in content {
            if lines.contains('[') {
                for pair in lines
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
                {
                    let stack = (pair.0 - 1) / 4;
                    //println!("pair: {pair:?} , stack: {stack}");
                    while aoc.ship.len() < stack + 1 {
                        aoc.ship.push(VecDeque::new());
                    }
                    aoc.ship[stack].push_front(pair.1)
                }
            }

            if lines.contains("move") {
                let words = lines.split_whitespace().collect::<Vec<&str>>();
                //println!("{:?}", words);
                if lines.contains("move") {
                    let amount: usize = words[1].parse().unwrap();
                    let from: usize = words[3].parse::<usize>().unwrap() - 1;
                    let to: usize = words[5].parse::<usize>().unwrap() - 1;
                    aoc.instructions.push(Move { amount, from, to });
                }
            }
        }
        //println!("{:?}", aoc);

        let mut ship1 = aoc.ship.clone();
        let mut ship2 = aoc.ship.clone();

        // move crates from stack to stack
        for i in &aoc.instructions {
            for _ in 0..i.amount {
                let temp = ship1[i.from].pop_back().unwrap();
                ship1[i.to].push_back(temp);
            }
        }
        let mut answer = "".to_string();
        for stack in aoc.ship {
            answer.push(*stack.back().unwrap());
        }

        // part two
        for i in aoc.instructions {
            let split_point = ship2[i.from].len() - i.amount;
            let removed = ship2[i.from].split_off(split_point);
            ship2[i.to].extend(removed);
        }

        let mut answer_part_two = "".to_string();
        for stack in ship2 {
            answer_part_two.push(*stack.back().unwrap());
        }

        answer_part_two
    }
}

// fn parse_input(input: Vec<String>) {
//     for lines in input {
//         let mut _v: Vec<char> = Vec::new();
//         if lines.contains('[') {
//             let _line = lines
//                 .as_bytes()
//                 .chunks(4)
//                 .map(std::str::from_utf8)
//                 .map(|v| v.unwrap().trim())
//                 .collect::<Vec<&str>>();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::*;

    #[test]
    fn test_day5_part1() {
        let one = five::Five {};
        let input =
            std::fs::read_to_string("./input/day5_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("", result);
    }
    #[test]
    fn test_day5_part2() {
        let one = five::Five {};
        let input =
            std::fs::read_to_string("./input/day5_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("", result);
    }
}
