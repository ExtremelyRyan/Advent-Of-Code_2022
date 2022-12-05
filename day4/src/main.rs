use std::{fs::read_to_string, path::Path};

fn main() {
    println!("Hello, world!");
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

fn part_one(lines: Vec<String>) -> i32{

    let mut pairs = 0;

    for line in lines {

        let (left, right) = line.split_once(',').unwrap();
        //println!("left: {left}, right: {right}");

        let (left_min, left_max) = left.split_once('-').unwrap();
        //println!("left_min: {left_min}, left_max: {left_max}");

        let (right_min , right_max) = right.split_once('-').unwrap();
        //println!("right_min: {right_min}, right_max: {right_max}");

        let lmin = left_min.trim().parse::<i32>().unwrap();
        let lmax = left_max.trim().parse::<i32>().unwrap();
        let rmin = right_min.trim().parse::<i32>().unwrap();
        let rmax = right_max.trim().parse::<i32>().unwrap();

        if (lmin <= rmin && lmax >= rmax)  || 
           (rmin <= lmin && rmax >= lmax) {
            pairs += 1;
        }
    }
    pairs
}

fn part_two(lines: Vec<String>) -> i32{

    let mut pairs = 0;

    for line in lines {

        let (left, right) = line.split_once(',').unwrap();
        //println!("left: {left}, right: {right}");

        let (left_min, left_max) = left.split_once('-').unwrap();
        //println!("left_min: {left_min}, left_max: {left_max}");

        let (right_min , right_max) = right.split_once('-').unwrap();
        //println!("right_min: {right_min}, right_max: {right_max}");

        let lmin = left_min.trim().parse::<i32>().unwrap();
        let lmax = left_max.trim().parse::<i32>().unwrap();
        let rmin = right_min.trim().parse::<i32>().unwrap();
        let rmax = right_max.trim().parse::<i32>().unwrap();

        if (lmin..=lmax).contains(&rmin) || (lmin..=lmax).contains(&rmax) || 
        (rmin..=rmax).contains(&lmin) || (rmin..=rmax).contains(&lmax)
        {
            println!("left: {left}, right: {right}");
            pairs += 1;
        }
    }
    pairs
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let lines = read_input("sample.txt");
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn test_parse_samples() {
        let lines = read_input("sample.txt");
        let pairs = part_one(lines);
        println!("test_parse_samples: {pairs}");
        assert_eq!(pairs, 2);
    }

    #[test]
    fn test_part_one() {
        let lines = read_input("input.txt");
        let pairs = part_one(lines);
        println!("test_parse: {pairs}");
        dbg!(pairs);
        // assert_eq!(pairs, 2);
    }

    #[test]
    fn test_part_two_samples() {
        let lines = read_input("sample2.txt");
        let pairs = part_two(lines);
        println!("test_parse: {pairs}"); 
        assert_eq!(pairs, 4);
    }

    #[test]
    fn test_part_two() {
        let lines = read_input("input.txt");
        let pairs = part_two(lines);
        println!("test_parse: {pairs}");
        dbg!(pairs);
        assert_eq!(pairs, 956);
    }
}
