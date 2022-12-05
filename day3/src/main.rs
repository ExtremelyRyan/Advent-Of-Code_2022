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

fn parse_lines(lines: Vec<String>) -> Vec<(String, String)> {
    // store moves here in a vector<touple>
    let mut rucksack: Vec<(String, String)> = Vec::new();

    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);
        rucksack.push((String::from(left), String::from(right)));
        //println!("left: {left}, right: {right}");
    }
    rucksack
}

fn get_ascii_value(found: Vec<char>) -> i32 {
    for i in found {
        if i.is_uppercase() {
            return i as i32 - 38;
        }
        else {
            return i as i32 - 96;
        }
    }
    return 0;
}

fn part_one(rucksack: Vec<(String, String)>) -> i32 {
    // found holds items that are 'found' in both rucksacks
    let mut found: Vec<char> = Vec::new();

    for (left, right) in rucksack {
    let matches = left.chars()
    .into_iter()
    .filter(|&a| right.contains(a)).collect::<Vec<char>>();
    found.push(matches[0]);

    }
    println!("found: {found:?}");

    // get "value" for each item found
    let total = get_ascii_value(found);

    total
}

fn part_two<T: AsRef<Path>>(path: T) -> i32 {
// Every set of three lines in your list corresponds to a single group
// 
    let mut sum = 0;
    let mut index = 0;

    let lines = read_input(path);
    let length = lines.len();

    loop {
        if index >= length { break;}

        let comp1 = &lines[index];
        let comp2 = &lines[index+1];
        let comp3 = &lines[index+2];
        let shared = comp1.chars()
        .into_iter()
        .filter(|&a| comp2.contains(a) && comp3.contains(a))
        .collect::<Vec<char>>();

        if shared.len() > 0 {
            sum += get_ascii_value(shared);
        }
        index += 3;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_read_sample() {
        let test = read_input("sample.txt");
        println!("{:?}", test);
        assert_eq!(test.len(), 6);
    }

    #[test]
    fn test_parse_samples() {
        let test = read_input("sample.txt");
        //println!("{:?}", test);
        let rucksack = parse_lines(test);
        for r in &rucksack {
            println!("left: {}, right: {}", r.0, r.1);
        }
        assert!(rucksack[0].0 == "vJrwpWtwJgWr");
    }

    #[test]
    fn test_part_one_samples() {
        let test = read_input("sample.txt");
        //println!("{:?}", test);
        let rucksack = parse_lines(test);
        let result = part_one(rucksack);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_one() {
        let test = read_input("input.txt");
        //println!("{:?}", test);
        let rucksack = parse_lines(test);
        let result = part_one(rucksack);
        println!("result: {result}");
    }

    #[test]
    fn test_part_two_samples() {
        let result = part_two("sample.txt");
        println!("result: {result}");
        assert_eq!(result, 70);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("input.txt");
        dbg!(result);
        assert_eq!(result, 2602);
    }

}
