
 
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn main() {

    let test = aoc::input_to_vec_string("./input/day2_input.txt");
    let moves = parse_lines(test);
    let solution1 = part_one(moves.clone());
    println!("Part one: {solution1}");
    let solution2 = part_two(moves);
    println!("Part two: {solution2}");

}
 
fn parse_lines(lines: Vec<String>) -> Vec<(char, char)> {
    // store moves here in a vector<touple>
    let mut moves: Vec<(char, char)> = Vec::new();

    for line in lines {
        let (opponent, player) = line.split_once(' ').unwrap();
        moves.push((
            opponent.chars().next().unwrap(),
            player.chars().next().unwrap(),
        ));
        //println!("op: {opponent}, player: {player}");
    }
    moves
}

fn part_one(moves: Vec<(char, char)>) -> i32 {
    let mut score = 0;

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
            || player_score == 3 && opponent_score == 2  // scissors beat paper
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
    score
}

// X means lose, 
// Y means draw,
// Z means win. Good luck!"
fn part_two(moves: Vec<(char, char)>) -> i32 {
    let mut score: i32 = 0;

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
                if opponent_score == 1 { score += 3;}
                else if opponent_score == 2 { score += 1;}
                else {score += 2;}
                
            },
            // draw
            'Y' => {
                score += opponent_score + 3;
                
            },
            'Z' => { 
                if opponent_score == 1 { score += 2 + Result::Win as i32;}
                else if opponent_score == 2 { score += 3 + Result::Win as i32;}
                else {score += 1 + Result::Win as i32;}
            },
             _ => todo!(),
        };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let test = aoc::input_to_vec_string("./input/day2_input.txt");
        //println!("{:?}", test);
        assert_eq!(test.len(), 2500);
    }

    #[test]
    fn test_part_one_sample() {
        let test = aoc::input_to_vec_string("./input/day2_sample.txt");
        let result = parse_lines(test);
        assert_eq!(part_one(result),15);
        
    }

    #[test]
    fn test_part_two_sample() {
        let test = aoc::input_to_vec_string("./input/day2_sample.txt");
        let result = parse_lines(test);
        assert_eq!(part_two(result),12);
        
    }
}
