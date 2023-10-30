use problem::Problem;
mod days;

mod problem {
    pub trait Problem {
        fn part_one(&self, input: &str) -> String;
        fn part_two(&self, input: &str) -> String;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // dbg!(&args);
    let days_available = [1, 2, 3, 4, 5];

    match args[0] == "all" {
        true => {
            for n in days_available {
                let problem = day_to_problem(n).expect("unknown problem");
                let result = run_problem(
                    problem,
                    3,
                    format!("./aoc_15/input/day{}_input.txt", n).as_str(),
                );
                println!("day{} {}", n, result);
            }
        }
        false => {
            let mut part = 3;
            if args.get(1).is_some() {
                part = args[1].parse().unwrap();
            }
            let problem = day_to_problem(args[0].parse().expect("input error")).unwrap();
            let result = run_problem(
                problem,
                part,
                format!("./aoc_15/input/day{}_input.txt", args[0]).as_str(),
            );
            println!("{}", result);
        }
    }
}

/// Takes in user input number and returns a reference to the struct associated with
/// the AOC day problem.
/// day: 1-25
fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(days::day_one::DayOne {})),
        // 2 => Some(Box::new(days::two::Two {})),
        // 3 => Some(Box::new(days::three::Three {})),
        // 4 => Some(Box::new(days::four::Four {})),
        // 5 => Some(Box::new(days::five::Five {})),
        // 6 => Some(Box::new(days::two::Two {})),
        // 7 => Some(Box::new(days::two::Two {})),
        // 8 => Some(Box::new(days::two::Two {})),
        // 9 => Some(Box::new(days::two::Two {})),
        // 10 => Some(Box::new(days::two::Two {})),
        // 11 => Some(Box::new(days::two::Two {})),
        // 12 => Some(Box::new(days::two::Two {})), 
        // ...
        _ => {
            println!("invalid selection. please enter correct day");
            std::process::exit(0);
        }
    }
}

/// Takes in a Problem Struct to run that day's solution
fn run_problem(problem: Box<dyn Problem>, parts: usize, input: &str) -> String {
    let input =
        std::fs::read_to_string(input).unwrap_or_else(|_| panic!("\nerror reading {}\n", input));
    match parts {
        1 => problem.part_one(&input),
        2 => problem.part_two(&input),
        3 => {
            format!(
                "Part 1: {} \t\t Part 2: {}",
                problem.part_one(&input),
                problem.part_two(&input)
            )
        }
        _ => "invalid part selected. Please choose 1-3".to_string(),
    }
}
