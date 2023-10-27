use crate::problem::Problem;

#[derive(Debug)]
pub struct One {}

impl Problem for One {
    fn part_one(&self, input: &str) -> String {
        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_1() {
        let test = aoc::input_to_vv_char("./input/day2_input.txt");
        //println!("{:?}", test);
        assert_eq!(test.len(), 2500);
    }
}
