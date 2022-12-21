#![allow(dead_code, unused)]
use std::{path::Path, fs::read_to_string, collections::VecDeque};


#[derive(Default, Debug)]
pub struct Day5 {
    ship: Vec<VecDeque<char>>,
    instructions: Vec<Move>,
}

impl Day5 {
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

fn main() {
    let lines = read_input("./input/day5_input.txt"); 
    func(lines);
}

/// read file, and return values within a Vector of Strings.
fn read_input<T: AsRef<Path>>(path: T) -> Vec<String> {
    read_to_string(path)
        .expect("Can't open/read file!")
        .split("\n")
        .filter(|s| !s.is_empty()) // so long as the string is not empty
        .map(|s| s.to_string()) // convert item to a string.
        .collect()
}

fn parse_input(input: Vec<String>) {

    for lines in input {
        let mut _v: Vec<char> = Vec::new();
        if lines.contains('[') {
            
            let line = lines.as_bytes().chunks(4)
            .map(std::str::from_utf8)
            .map(|v| v.unwrap().trim())
            .collect::<Vec<&str>>();
            //println!("{line:?}");


        }

    }
}

/*
https://github.com/UncleScientist/aoclib-rs/blob/4adb0939afb1de0d9c62587c6b9b494cb663fadf/src/bin/run-aoc/aoc2022/aoc2022_05.rs#L22
*/
fn func(input: Vec<String>) {

    let mut aoc = Day5::new();

    for lines in input { 

        if lines.contains('[') {
            for pair in lines.chars().enumerate().filter(|(i,c)| *c != ' ' && (i + 3) % 4 == 0)
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
            aoc.instructions.push(Move{amount, from, to});
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

    println!("part one: {answer}");

    // part two
    for i in aoc.instructions  {

        let split_point = ship2[i.from].len() - i.amount;
        let removed = ship2[i.from].split_off(split_point);
        ship2[i.to].extend(removed);
    }

    let mut answer_part_two = "".to_string();
    for stack in ship2{
        answer_part_two.push(*stack.back().unwrap());
    }

    println!("part two: {answer_part_two}");

}



#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn read_sample_input() {
        let lines = read_input("./input/day5_sample.txt"); 
        assert_eq!(lines.len(), 9);
    }

    #[test]
    fn parse_sample() {
        let lines = read_input("./input/day5_sample.txt"); 
        parse_input(lines);
        //assert_eq!(lines.len(), 9);
    }

    #[test]
    fn func_test() {
        let lines = read_input("./input/day5_sample.txt"); 
        func(lines); 
    }

}