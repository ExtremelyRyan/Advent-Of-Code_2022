use std::{path::Path, fs::read_to_string};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


/// read file, and return values within a Vector of Strings.
pub fn input_to_vec_string<T: AsRef<Path>>(path: T) -> Vec<String> {
    read_to_string(path)
        .expect("Can't open/read file!")
        .split("\n")
        .filter(|s| !s.is_empty()) // so long as the string is not empty
        .map(|s| s.to_string()) // convert item to a string.
        .collect()
}
/// converts standard text to a vector of vectors (char), stripping out empty lines and non alphabetic characters.
pub fn input_to_vv_char<T: AsRef<Path>>(path: T) -> Vec<Vec<char>> {
    let s: Vec<String> = read_to_string(path)
        .expect("Can't open/read file!")
        .split("\n")
        .filter(|s| !s.is_empty()) // so long as the string is not empty
        .map(|s| s.to_string()) // convert item to a string.
        .collect();

    let mut ch: Vec<Vec<char>> = Vec::new();

    for s in s.iter() {
        let mut v = Vec::new();
        for c in s.chars().filter(|s| s.is_alphabetic()) {
            v.push(c);
        }
        ch.push(v);
    }
    ch

}