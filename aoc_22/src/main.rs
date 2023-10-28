use problem::Problem;
mod days;

mod problem {
    pub trait Problem {
        fn part_one(&self, input: &str) -> String;
        fn part_two(&self, input: &str) -> String;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let days_available = [1, 2, 3];

    if args[1] == "all" {
        for n in days_available {
            let problem = day_to_problem(n).expect("unknown problem");
            println!(
                "{}",
                run_problem(
                    problem,
                    3,
                    format!("./aoc_22/input/day{}_input.txt", n).as_str()
                )
            );
        }
    } else {
        let problem = day_to_problem(args[1].parse().expect("input error")).unwrap();
        println!(
            "{}",
            run_problem(
                problem,
                3,
                format!("./aoc_22/input/day{}_input.txt", args[1]).as_str()
            )
        );
    }
}

/// Takes in user input number and returns a reference to the struct associated with
/// the AOC day problem.
/// day: 1-25
fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(days::one::One {})),
        2 => Some(Box::new(days::one::One {})),
        3 => Some(Box::new(days::three::Three {})),
        // 4 => Some(Box::new(days::four::Four {})),
        // 5 => Some(Box::new(days::five::Five {})),
        // 6 => Some(Box::new(days::two::Two {})),
        // 7 => Some(Box::new(days::two::Two {})),
        // 8 => Some(Box::new(days::two::Two {})),
        // 9 => Some(Box::new(days::two::Two {})),
        // 10 => Some(Box::new(days::two::Two {})),
        // 11 => Some(Box::new(days::two::Two {})),
        // 12 => Some(Box::new(days::two::Two {})),
        // 3 => Some(Box::new(DayThree {})),
        // ...
        _ => None,
    }
}

/// Takes in a Problem Struct to run that day's solution
fn run_problem(problem: Box<dyn Problem>, parts: usize, input: &str) -> String {
    let input = std::fs::read_to_string(input).expect("error reading from file!");
    match parts {
        1 => problem.part_one(&input),
        2 => problem.part_two(&input),
        3 => {
            format!(
                "Part 1: {}\nPart 2: {}",
                problem.part_one(&input),
                problem.part_two(&input)
            )
        }
        _ => "invalid part selected. Please choose 1-3".to_string(),
    }
}
