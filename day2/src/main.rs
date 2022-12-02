use std::fs;

enum Plays {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

use std::collections::HashMap;

let hash_move = HashMap::from([
    ("A" | "X" | "Rock", 1),
    ("B" | "Y" | "Paper", 2),
    ("C" | "Z" | "Scissors", 3),
]);

let hash_result = HashMap::from([
    (hash_move::Rock, hash_move::Rock, Outcome::Draw),
    (hash_move::Rock, hash_move::Paper, Outcome::Lost),
    (hash_move::Rock, hash_move::Scissors, Outcome::Win),

    (hash_move::Paper, hash_move::Rock, Outcome::Win),
    (hash_move::Paper, hash_move::Paper, Outcome::Draw),
    (hash_move::Paper, hash_move::Scissors, Outcome::Lost),

    (hash_move::Scissors, hash_move::Rock, Outcome::Lost),
    (hash_move::Scissors, hash_move::Paper, Outcome::Win),
    (hash_move::Scissors, hash_move::Scissors, Outcome::Draw),

]);





fn main() {
    let file = fs::File::open("input.txt");


}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}