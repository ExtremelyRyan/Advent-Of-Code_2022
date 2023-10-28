use crate::days::utility;
use crate::problem::Problem;

#[derive(Debug)]
pub struct Two {}

enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Problem for Two {
    fn part_one(&self, input: &str) -> String {
        let mut score = 0;
        let moves = parse_lines(input.to_string());

        for m in &moves {
            let opponent_score: i32 = match m.0 {
                'A' => 1, // Rock
                'B' => 2, // Paper
                'C' => 3, // Scissors
                _ => todo!(),
            };
            let player_score: i32 = match m.1 {
                'X' => 1, // Rock
                'Y' => 2, // Paper
                'Z' => 3, // Scissors
                _ => todo!(),
            };

            // win
            if player_score == 1 && opponent_score == 3      // rock beats scissors
                || player_score == 2 && opponent_score == 1  // Paper beats rock
                || player_score == 3 && opponent_score == 2
            // scissors beat paper
            {
                score += player_score + Result::Win as i32;
            }
            //draw
            else if opponent_score == player_score {
                score += player_score + Result::Draw as i32;
            }
            // loss
            else {
                score += player_score + Result::Lose as i32;
            }
            //println!("DEBUG: score: {score}");
        }
        score.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut score: i32 = 0;

        let moves = parse_lines(input.to_string());

        for m in &moves {
            let opponent_score: i32 = match m.0 {
                'A' => 1, // Rock
                'B' => 2, // Paper
                'C' => 3, // Scissors
                _ => todo!(),
            };

            match m.1 {
                // lose
                'X' => {
                    if opponent_score == 1 {
                        score += 3;
                    } else if opponent_score == 2 {
                        score += 1;
                    } else {
                        score += 2;
                    }
                }
                // draw
                'Y' => {
                    score += opponent_score + 3;
                }
                'Z' => {
                    if opponent_score == 1 {
                        score += 2 + Result::Win as i32;
                    } else if opponent_score == 2 {
                        score += 3 + Result::Win as i32;
                    } else {
                        score += 1 + Result::Win as i32;
                    }
                }
                _ => todo!(),
            };
        }
        score.to_string()
    }
}

fn parse_lines(lines: String) -> Vec<(char, char)> {
    let collection: Vec<String> = utility::input_to_vec_string(lines);

    // store moves here in a vector<touple>
    let mut moves: Vec<(char, char)> = Vec::new();

    for line in collection {
        let (opponent, player) = line.split_once(' ').unwrap();
        moves.push((
            opponent.chars().next().unwrap(),
            player.chars().next().unwrap(),
        ));
        //println!("op: {opponent}, player: {player}");
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::*;

    #[test]
    fn test_day2_part1() {
        let one = two::Two {};
        let input =
            std::fs::read_to_string("./input/day2_input.txt").expect("error reading from file!");
        let result = one.part_one(&input);
        assert_eq!("10718", result);
    }
    #[test]
    fn test_day2_part2() {
        let one = two::Two {};
        let input =
            std::fs::read_to_string("./input/day2_input.txt").expect("error reading from file!");
        let result = one.part_two(&input);
        assert_eq!("14652", result);
    }
}
