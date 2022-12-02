use std::io::{self, BufRead};

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
    let mut scores: u32 = 0;
    let mut counter = 0;

    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        let mut line_split = line.split_whitespace();
        let opponent_choice =
            RPS::from_opponent_char(&line_split.next().unwrap().chars().next().unwrap());
        let player_choice =
            RPS::from_player_char(&line_split.next().unwrap().chars().next().unwrap());
        scores += player_choice.unwrap().score(&opponent_choice.unwrap());
        counter += 1;
    }
    println!("rounds: {}", counter);
    println!("score: {}", scores);
}
