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
    dbg!(&args);

    let res = day_to_problem(args[0].parse().expect("input error")).unwrap();

    println!("{}", res.part_one("hello"));
}

/// Takes in user input number and returns a reference to the struct associated with
/// the AOC day problem.
/// day: 1-25
fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(days::one::One {})),
        // 2 => Some(Box::new(DayTwo {})),
        // 3 => Some(Box::new(DayThree {})),
        // ...
        _ => None,
    }
}

/// Takes in a Problem Struct to run that day's solution
///
fn run_problem(problem: Box<dyn Problem>, parts: usize) -> String {}
