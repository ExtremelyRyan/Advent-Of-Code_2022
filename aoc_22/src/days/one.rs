use crate::problem::Problem;

#[derive(Debug)]
pub struct One {}

impl Problem for One {
    fn part_one(&self, input: &str) -> String {
        // Initialize two integer variables - current and max
        let mut current = 0;
        let mut max = 0;

        // Iterate over each line in the 'reader'.
        // The 'lines' method returns an iterator over the lines in the 'reader'.
        for line in input.lines() {
            // println!("{}", line);
            // If line is empty, figure out if we have a new max
            if line.is_empty() {
                if current > max {
                    max = current;
                }
                current = 0;
                continue;
            }

            // Parse the line as an 'i32' (32-bit signed integer) and bind the
            // result to the 'number' variable.

            // If an error occurs, print it and continue
            // Start that code here:
            match line.parse::<i32>() {
                Ok(n) => {
                    current += n;
                }
                Err(e) => {
                    println!("Part One Error: {}", e);
                    continue;
                }
            };
        }
        format!("{}", max)
    }

    fn part_two(&self, input: &str) -> String {
        // Initialize two integer variables - current and max
        let mut current = 0;
        let mut elves = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                elves.push(current);
                current = 0;
                continue;
            }
            match line.parse::<i32>() {
                Ok(n) => current += n,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
        }

        // now that we have the elves in one vector, sort them by largest calorie count
        elves.sort_by(|a, b| b.cmp(a));

        // dbg!(&elves);

        // get top 3 results
        let top_three_elves = &elves[0..3];

        //print results.
        let mut sum = 0;
        top_three_elves.iter().for_each(|elf| sum += elf);
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::one;

    use super::*;

    #[test]
    fn test_day1_part1() {
        let one = one::One {};
        let input =
            std::fs::read_to_string("./input/day1_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("71924", result);
    }
    #[test]
    fn test_day1_part2() {
        let one = one::One {};
        let input =
            std::fs::read_to_string("./input/day1_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("210406", result);
    }
}
