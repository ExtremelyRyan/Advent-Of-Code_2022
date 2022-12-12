use std::{fs::{File}, io::{BufReader, BufRead}};


#[derive(Debug, Default)]
struct Monkey { 
    item: Vec<Items>,
}
#[derive(Debug, Default)]
struct Items {
    worry: Vec<i32>,
    operation: (bool, bool),
    test: i32,
    throw: (i32,i32),
}


impl Monkey {
    pub fn new() -> Self {
        Self { item: Vec::new() }
    }
}

impl Items {
    pub fn new() -> Self {
        Self { worry: vec![], operation: (false,false), test: 0, throw: (0,0) }
    }
}
fn main() {
    println!("Hello, world!");
}

fn parse(path: String)  {
    let lines = std::fs::read_to_string(path);
    let s: Vec<&str> = lines.unwrap().split_ascii_whitespace().collect();

    for ss in s.iter() {
        println!("{ss}");
    }
}

fn clean(input : Vec<&str>) {

    let mut m = Monkey::default();

    for line in input.iter() {

        if line.contains("Monkey") {
            m = Monkey::default();
        }

        let mut itm = Items::default();



    }


    // let mut monke: Vec<&Monkey> = Vec::new();

    // let mut m = Monkey::new();

    // for line in input.iter() {
    //     if line.contains("Monkey") {
    //         let mut m = Monkey::new();
    //     }
    //     if line.contains("Starting") {
    //         let v: Vec<&str> = line.split(' ').collect();

    //          for x in v.iter() {
    //             if x.parse::<i32>().is_ok() {
    //                 let n = x.parse::<i32>().unwrap();
    //                 m.items.push(n);
    //             }
    //         }
    //     }
    //     monke.push(&m);
    //     println!("Monkey: {:?}", &m);
    // }

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