use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_opponent_char(c: &char) -> Result<Self, String> {
        match c {
            'A' => Ok(RPS::Rock),
            'B' => Ok(RPS::Paper),
            'C' => Ok(RPS::Scissors),
            _ => Err(format!("{} is not a correct player choice!", c)),
        }
    }

    fn from_player_char(c: &char) -> Result<Self, String> {
        match c {
            'X' => Ok(RPS::Rock),
            'Y' => Ok(RPS::Paper),
            'Z' => Ok(RPS::Scissors),
            _ => Err(format!("{} is not a correct opponent choice!", c)),
        }
    }

    fn from_outcome(c: &char, opponent_choice: &RPS) -> Result<Self, String> {
        match c {
            'X' => match opponent_choice {
                RPS::Rock => Ok(RPS::Scissors),
                RPS::Paper => Ok(RPS::Rock),
                RPS::Scissors => Ok(RPS::Paper),
            },
            'Y' => Ok(opponent_choice.clone()),
            'Z' => match opponent_choice {
                RPS::Rock => Ok(RPS::Paper),
                RPS::Paper => Ok(RPS::Scissors),
                RPS::Scissors => Ok(RPS::Rock),
            },
            _ => Err(format!("{} is not a correct opponent choice!", c)),
        }
    }

    fn score(&self, opponent_choice: &RPS) -> u32 {
        match *self {
            RPS::Rock => {
                1 + match *opponent_choice {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                }
            }
            RPS::Paper => {
                2 + match *opponent_choice {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0,
                }
            }
            RPS::Scissors => {
                3 + match *opponent_choice {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3,
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scores_part1: u32 = 0;
    let mut scores_part2: u32 = 0;
    let mut counter = 0;

    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        let mut line_split = line.split_whitespace();
        let first_char = line_split.next().unwrap().chars().next().unwrap();
        let second_char = line_split.next().unwrap().chars().next().unwrap();
        let opponent_choice = RPS::from_opponent_char(&first_char).unwrap();
        let player_choice_part1 = RPS::from_player_char(&second_char);
        let player_choice_part2 = RPS::from_outcome(&second_char, &opponent_choice);
        scores_part1 += player_choice_part1.unwrap().score(&opponent_choice);
        scores_part2 += player_choice_part2.unwrap().score(&opponent_choice);
        counter += 1;
    }
    println!("rounds: {}", counter);
    println!("score part 1: {}", scores_part1);
    println!("score part 2: {}", scores_part2);
}
