use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut elves: Vec<u32> = vec![0];
    let mut len_elves = 1;

    for rline in stdin.lock().lines() {
        if let Ok(line) = rline {
            if let Ok(calories) = line.parse::<u32>() {
                elves[len_elves - 1] += calories;
            } else {
                elves.push(0);
                len_elves += 1;
            }
        }
    }
    elves.sort();
    elves.reverse();

    println!("{}", elves[0] + elves[1] + elves[2]);

}
