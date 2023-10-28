use crate::problem::Problem; 
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug)]
pub struct Seven {}

impl Problem for Seven {
    fn part_one(&self, input: &str) -> String {
        let file = File::open("./aoc_22/input/day7_input.txt").expect("d7 file error");
        let mut reader = BufReader::new(file).lines();
        let mut directories: Vec<i32> = Vec::new(); 
    
        let part1: i32 = directories.iter().fold(0, |acc, directory| {
            acc + if *directory <= 100000 { *directory } else { 0 }
        });  
        part1.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let file = File::open("./aoc_22/input/day7_input.txt").expect("d7 file error");
        let mut reader = BufReader::new(file).lines();
        let mut directories: Vec<i32> = Vec::new();
        let total = directory_size(&mut reader, &mut directories);
    
        let upgrade_size = 30000000;
        let max_space = 70000000;
        let target_space = upgrade_size - (max_space - total);
    
        let part2: i32 = directories.iter().fold(total, |acc, directory| {
            if *directory <= acc && *directory >= target_space {
                *directory
            } else {
                acc
            }
        });
        part2.to_string()
    }
}


fn directory_size(reader: &mut Lines<BufReader<File>>, list: &mut Vec<i32>) -> i32 {
    let mut size: i32 = 0;

    loop {
        let line = reader.next();
        println!("line: {line:?}");
        match line {
            None => break,
            _ => (),
        };

        let line = line.unwrap().unwrap();
        if line == "$ cd .." {
            break;
        }
        let words: Vec<&str> = line.split(" ").collect();
        if words[1] == "cd" {
            size += directory_size(reader, list);
        }
        size += words[0].parse::<i32>().unwrap_or(0);
    }
    list.push(size);

    size
}

#[cfg(test)]
mod tests {
    use crate::days::*;
    use super::*;

    #[test]
    fn test_day7_part1() {
        let one = seven::Seven {};
        let input =
            std::fs::read_to_string("./input/day7_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("", result);
    }
    #[test]
    fn test_day7_part2() {
        let one = seven::Seven {};
        let input =
            std::fs::read_to_string("./input/day7_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("", result);
    }
}
