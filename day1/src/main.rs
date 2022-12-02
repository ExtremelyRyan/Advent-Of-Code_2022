use std::fs::File;
use std::io::{self,BufReader, BufRead};

// Define the 'main' function.
// This is the entry point of the program.
// The 'main' function returns a 'Result<(), io::Error>', which is
// an enumeration that represents either a successful value of type '()'
// or an error value of type 'io::Error'.
fn main() -> io::Result<()> {

    part1();
    part2()
}
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn part1() -> io::Result<()> { 
    let f = File::open("day1_1.txt")?;
    let reader = BufReader::new(f);

    // Initialize two integer variables - current and max
    let mut current = 0;
    let mut max = 0;

    // Iterate over each line in the 'reader'.
    // The 'lines' method returns an iterator over the lines in the 'reader'.
    for line in reader.lines() {
        // unwrap line for whatever reason.
        let line = line?; 

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
            },
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }
 
    println!("Max: {}", max);
    Ok(())
}

// Find the top three Elves carrying the most Calories. 
// How many Calories are those Elves carrying in total?
fn part2() -> io::Result<()>  {
    let f = File::open("day1_1.txt")?;
    let reader = BufReader::new(f);

    // Initialize two integer variables - current and max
    let mut current = 0;
    let mut elves = Vec::new();

    for line  in reader.lines() {
        let line = line?;

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
    elves.sort_by(|a,b| b.cmp(a));

    // dbg!(&elves);

    // get top 3 results
    let top_three_elves = &elves[0..3];

    //print results.
    let mut sum = 0;
    top_three_elves.into_iter().for_each(|elf| sum += elf);
    println!("top 3 total: {}", sum);
    
    Ok(())
}