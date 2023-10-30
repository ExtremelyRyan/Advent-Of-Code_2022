use crate::problem::Problem;
use super::utility;

#[derive(Debug)]
pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        // dbg!(&input); 
        let mut floor = 0;

        for c in input.chars(){
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => ()
            }
        } 
        floor.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        String::from("incomplete")
    }
}

#[cfg(test)]
mod tests {
    use crate::days::*;
    use super::*;

    #[test]
    fn test_dayX_part1() {
        let one = DayOne {};
        let input =
            std::fs::read_to_string("./input/day1_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("", result);
    }
    #[test]
    fn test_dayX_part2() {
        let one = DayOne {};
        let input =
            std::fs::read_to_string("./input/day1_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("", result);
    }
}
