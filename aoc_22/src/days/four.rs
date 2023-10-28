use super::utility;
use crate::problem::Problem;

#[derive(Debug)]
pub struct Four {}

impl Problem for Four {
    fn part_one(&self, input: &str) -> String {
        let lines = utility::string_to_vec_string(input);
        let mut pairs = 0;

        for line in lines {
            let (left, right) = line.split_once(',').unwrap();
            //println!("left: {left}, right: {right}");

            let (left_min, left_max) = left.split_once('-').unwrap();
            //println!("left_min: {left_min}, left_max: {left_max}");

            let (right_min, right_max) = right.split_once('-').unwrap();
            //println!("right_min: {right_min}, right_max: {right_max}");

            let lmin = left_min.trim().parse::<i32>().unwrap();
            let lmax = left_max.trim().parse::<i32>().unwrap();
            let rmin = right_min.trim().parse::<i32>().unwrap();
            let rmax = right_max.trim().parse::<i32>().unwrap();

            if (lmin <= rmin && lmax >= rmax) || (rmin <= lmin && rmax >= lmax) {
                pairs += 1;
            }
        }
        pairs.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let lines = utility::string_to_vec_string(input);
        let mut pairs = 0;

        for line in lines {
            let (left, right) = line.split_once(',').unwrap();
            //println!("left: {left}, right: {right}");

            let (left_min, left_max) = left.split_once('-').unwrap();
            //println!("left_min: {left_min}, left_max: {left_max}");

            let (right_min, right_max) = right.split_once('-').unwrap();
            //println!("right_min: {right_min}, right_max: {right_max}");

            let lmin = left_min.trim().parse::<i32>().unwrap();
            let lmax = left_max.trim().parse::<i32>().unwrap();
            let rmin = right_min.trim().parse::<i32>().unwrap();
            let rmax = right_max.trim().parse::<i32>().unwrap();

            if (lmin..=lmax).contains(&rmin)
                || (lmin..=lmax).contains(&rmax)
                || (rmin..=rmax).contains(&lmin)
                || (rmin..=rmax).contains(&lmax)
            {
                //println!("left: {left}, right: {right}");
                pairs += 1;
            }
        }
        pairs.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::*;

    #[test]
    fn test_day4_part1() {
        let one = four::Four {};
        let input =
            std::fs::read_to_string("./input/day4_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("651", result);
    }
    #[test]
    fn test_day4_part2() {
        let one = four::Four {};
        let input =
            std::fs::read_to_string("./input/day4_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("956", result);
    }
}
