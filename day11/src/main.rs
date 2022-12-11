use std::{fs::{File}, io::{BufReader, BufRead}};


#[derive(Debug)]
struct Monkey { 
    items: Vec<i32>,
    operation: i32,
    div: i32,
    true_cond: i32,
    false_cond: i32,
}

impl Monkey {
    pub fn default() -> Monkey {
        Self { items: vec![], operation: 0, div: 0, true_cond: 0, false_cond: 0 }
    }
    pub fn new() -> Monkey {
        Monkey::default()
    }

}

fn main() {
    println!("Hello, world!");
}

fn parse(path: String) -> Vec<String> {
    let f = File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut v = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        //println!("Line: {}", &line);
        v.push(line);
    }
    v
}

fn clean(input : Vec<String>) {

    let mut monke: Vec<Monkey> = Vec::new();
    let mut m = Monkey::new();
    for line in input.iter() {
        if line.contains("Monkey") {
            let mut m = Monkey::new();
        }
        if line.contains("Starting") {
            let v: Vec<&str> = line.split(' ').collect();

            let nums = Vec::new();
             for x in v.iter() {
                if x.parse::<i32>().is_ok() {
                   nums.push(x.parse::<i32>().unwrap());
                }
            }
            m.items.push(nums);
        }
        monke.push(m);
        println!("Monkey: {:?}", &m);
    }

}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let lines = parse(String::from("sample.txt"));
        assert_eq!(lines.len(), 27);

        clean(lines);
    }

}