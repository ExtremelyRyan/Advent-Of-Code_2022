use crate::problem::Problem; 
use std::collections::HashMap;

#[derive(Debug)]
pub struct Six {}

impl Problem for Six {
    fn part_one(&self, input: &str) -> String {
        let _ = input;
        String::from("incomplete")
    }
    fn part_two(&self, input: &str) -> String {
        let _ = input;
        String::from("incomplete")
    }
}

fn find_duplicate_chars(str_in: String, offset: usize) -> usize {
    let mut letter: HashMap<char, char> = HashMap::new();
    let mut counter: usize = 0;

    while counter < str_in.len() {
        let mut duplicate: bool = false;
        let s = &str_in[counter..counter + offset];
        //println!("{s}");
        //println!("Range: {} to {}",counter, counter+4);

        for c in s.chars() {
            if letter.contains_key(&c) {
                duplicate = true;
                //println!("Duplicate found!: {c} in chunk: {s}");
            } else {
                letter.insert(c, c);
            }
        }
        if !duplicate {
            return counter + offset;
        }
        letter.clear();
        counter += 1;
    }
    0
}

 

#[cfg(test)]
mod tests {
    use crate::days::*;
    use super::*;

    #[test]
    fn test_day6_part1() {
        let one = six::Six {};
        let input =
            std::fs::read_to_string("./input/day6_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("", result);
    }
    #[test]
    fn test_day6_part2() {
        let one = six::Six {};
        let input =
            std::fs::read_to_string("./input/day6_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("", result);
    }

    // unconvered tests
    // --------------------
    // #[test]
    // fn test_read_input() {
    //     let result = read_input("./input/day6_input.txt");
    //     assert_eq!(result.len(), 4095);
    // }

    // #[test]
    // fn test_duplicates() {
    //     let test_str1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(); // first marker after character 5
    //     let test_str2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string(); // first marker after character 6
    //     let test_str3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(); // first marker after character 10
    //     let test_str4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(); // first marker after character 11
    //     assert_eq!(find_duplicate_chars(test_str1, 4), 5);
    //     assert_eq!(find_duplicate_chars(test_str2, 4), 6);
    //     assert_eq!(find_duplicate_chars(test_str3, 4), 10);
    //     assert_eq!(find_duplicate_chars(test_str4, 4), 11);
    // }

    // #[test]
    // fn test_input() {
    //     let s = read_input("./input/day6_input.txt");
    //     let marker = find_duplicate_chars(s, 4);
    //     println!("part one marker: {marker}");
    //     assert_eq!(marker, 1542);
    // }

    // #[test]
    // fn test_duplicates_part2() {
    //     assert_eq!(
    //         find_duplicate_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
    //         19
    //     );
    //     assert_eq!(
    //         find_duplicate_chars("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
    //         23
    //     );
    //     assert_eq!(
    //         find_duplicate_chars("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
    //         23
    //     );
    //     assert_eq!(
    //         find_duplicate_chars("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
    //         29
    //     );
    //     assert_eq!(
    //         find_duplicate_chars("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
    //         26
    //     );
    // }

    // #[test]
    // fn test_input_part2() {
    //     let s = read_input("./input/day6_input.txt");
    //     let marker = find_duplicate_chars(s, 14);
    //     println!("part two marker: {marker}");
    //     assert_eq!(marker, 3153);
    // }
}
