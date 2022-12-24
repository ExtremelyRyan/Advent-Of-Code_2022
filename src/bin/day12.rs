use aoc::*;

fn main() {
    println!("Day 12 not complete")

    // find locations of S, E
    
    
}

fn get_ascii_value(ch: char) -> i32 {
    ch as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() { 
        let test = aoc::input_to_vv_char("./input/day12_sample.txt");
       //println!("{:?}", test);
        assert_eq!(test.len(), 5);
    }

    #[test]
    fn test_input() {
        let test = aoc::input_to_vv_char("./input/day12_input.txt");
        //println!("{:?}", test);
        assert_eq!(test.len(), 41);
    }

}