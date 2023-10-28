use crate::problem::Problem;
use super::utility;

#[derive(Debug)]
pub struct Three {}

impl Problem for Three {
    fn part_one(&self, input: &str) -> String {
    // found holds items that are 'found' in both rucksacks
    let mut found: Vec<char> = Vec::new();
    let rucksack = parse_lines(utility::input_to_vec_string(input.to_string()));

    for (left, right) in rucksack {
        let matches = left
            .chars()
            .into_iter()
            .filter(|&a| right.contains(a))
            .collect::<Vec<char>>();
        found.push(matches[0]);
    }
    //println!("found: {found:?}");

    // get "value" for each item found
    let total = get_ascii_value(found);

    total.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let lines =  utility::input_to_vec_string(input.to_string());
// Every set of three lines in your list corresponds to a single group
    //
    let mut sum = 0;
    let mut index = 0; 
    let length = lines.len();

    loop {
        if index >= length {
            break;
        }

        let comp1 = &lines[index];
        let comp2 = &lines[index + 1];
        let comp3 = &lines[index + 2];
        let shared = comp1
            .chars()
            .into_iter()
            .filter(|&a| comp2.contains(a) && comp3.contains(a))
            .collect::<Vec<char>>();

        if shared.len() > 0 {
            sum += get_ascii_value(shared);
        }
        index += 3;
    }
    sum.to_string()

    }
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
        } else {
            return i as i32 - 96;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::days::*;
    use super::*;

    #[test]
    fn test_day3_part1() {
        let one = three::Three {};
        let input =
            std::fs::read_to_string("./input/day3_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("17", result);
    }
    #[test]
    fn test_day3_part2() {
        let one = three::Three {};
        let input =
            std::fs::read_to_string("./input/day3_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("2602", result);
    }
}
