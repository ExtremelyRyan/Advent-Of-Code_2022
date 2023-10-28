// --- Day 10: Cathode-Ray Tube ---
#![allow(dead_code)]
use std::{fs::read_to_string, path::Path};

fn main() {
    // let p = parse("./input/day10_input.txt");
    // println!("part one: {}", part_one(p));
    test_part1();
    test_part2()
}

fn parse<T: AsRef<Path>>(path: T) -> Vec<(String, String)> {
    let s = read_to_string(path).expect("Can't open/read file!");

    let mut v: Vec<(String, String)> = Vec::new();

    for l in s.lines() {
        if l.contains("noop") {
            v.push(("noop".to_string(), "".to_string()));
        } else {
            let (instruction, n) = l.split_once(' ').unwrap_or_default();
            v.push((instruction.to_string(), n.to_string()));
        }
    }

    v
}

// part one:
// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles.
// What is the sum of these six signal strengths?

// Start by figuring out the signal being sent by the CPU.
// The CPU has a single register, X, which starts with the value 1.
// It supports only two instructions:
// addx -> two cycles, after X increased by value +-V
// noop -> one cycle, no other effect.
fn part_one(input: Vec<(String, String)>) -> i32 {
    let mut x = 1;
    let mut cycle = 0;

    // aggregate holds <cycle #, cycle * register x>
    let mut aggregate: Vec<i32> = Vec::new();

    for (instructions, val) in input.iter() {
        cycle += 1;

        //println!("cycle:{cycle}, instruction: {instructions}, value: {val}, register: {x}");

        if check_current_cycle(cycle) {
            //println!("cycle: {cycle}, aggregate: {}", cycle * x);
            aggregate.push(cycle * x);
        }

        if instructions == "noop" {
            continue;
        }

        if instructions == "addx" {
            cycle += 1;

            if check_current_cycle(cycle) {
                //println!("cycle: {cycle}, aggregate: {}", cycle * x);
                aggregate.push(cycle * x);
            }
            x += val.parse::<i32>().unwrap();
        }
    }
    aggregate.iter().sum()
    //x
}

fn part_two(input: Vec<(String, String)>) -> i32 {
    // Start by figuring out the signal being sent by the CPU.
    // The CPU has a single register, X, which starts with the value 1.
    // It supports only two instructions:

    // addx -> two cycles, after X increased by value +-V
    // noop -> one cycle, no other effect.

    let mut x = 1;
    let mut cycle = 0;
    //let crt: Vec<Vec<char>> = Vec::new();

    // aggregate holds <cycle #, cycle * register x>
    let mut aggregate: Vec<i32> = Vec::new();

    for (instructions, val) in input.iter() {
        cycle += 1;

        println!("cycle:{cycle}, instruction: {instructions}, value: {val}, register: {x}");

        if check_current_cycle(cycle) {
            println!("cycle: {cycle}, aggregate: {}", cycle * x);
            aggregate.push(cycle * x);
        }

        draw_pixel(cycle, x);

        if instructions == "noop" {
            continue;
        }

        if instructions == "addx" {
            cycle += 1;

            if check_current_cycle(cycle) {
                println!("cycle: {cycle}, aggregate: {}", cycle * x);
                aggregate.push(cycle * x);
            }
            x += val.parse::<i32>().unwrap();
        }
    }
    aggregate.iter().sum()
    //x
}

fn draw_pixel(cycle: i32, x: i32) {
    //let current_pos = cycle % 40;
    if cycle != x - 1 | x | x + 1 {}
}

fn check_current_cycle(cycle: i32) -> bool {
    cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220
}

fn values() -> Vec<isize> {
    let mut values = Vec::new();
    let mut x = 1;
    for line in include_str!("..\\..\\input\\day10_input.txt").lines() {
        values.push(x);
        if line != "noop" {
            values.push(x);
            let delta: isize = line.split_at(5).1.parse().unwrap();
            x += delta;
        }
    }
    values
}

fn test_part1() {
    let result: isize = values()
        .iter()
        .enumerate()
        .map(|(i, &x)| (isize::try_from(i).unwrap() + 1) * x)
        .skip(19)
        .step_by(40)
        .sum();
    println!("{result}");
}

fn test_part2() {
    let result: String = values()
        .iter()
        .zip((0..40).cycle())
        .map(|(value, position)| {
            if (value - position).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect();

    for (i, c) in result.chars().enumerate() {
        if i % 40 != 0 && i != 0 {
            print!("{c}");
        } else {
            println!("{c}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let v: Vec<(String, String)> = vec![
            (String::from("noop"), String::from("")),
            (String::from("addx"), String::from("3")),
            (String::from("addx"), String::from("-5")),
        ];
        assert_eq!(part_one(v), -1);
    }

    #[test]
    fn example2() {
        let p = parse("./input/day10_sample.txt");
        assert_eq!(part_one(p), 13140);
    }
    #[test]
    fn example3() {
        let p = parse("./input/day10_input.txt");
        assert_eq!(part_one(p), 14160);
    }
}
