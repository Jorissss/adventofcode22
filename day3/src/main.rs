use std::collections::HashSet;
use std::io::{self, BufRead};

fn score(c: u8) -> u32 {
    if c >= 65 && c <= 90 {
        return c as u32 - (65 - 27);
    } else if c >= 97 && c <= 122 {
        return c as u32 - 96;
    } else {
        panic!("{} is not a character between A-Z or a-z!", c);
    }
}

#[allow(dead_code)]
fn part1() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;

    for rline in stdin.lock().lines() {
        let mut compartment1 = rline.unwrap().into_bytes();
        let compartment2 = compartment1.split_off(compartment1.len() / 2);
        let compartment1_h: HashSet<u8> = compartment1.into_iter().collect();
        let doubles: HashSet<&u8> = compartment2
            .iter()
            .filter(|e| compartment1_h.contains(e))
            .collect();
        let doubles: Vec<&u8> = doubles.into_iter().collect();
        if doubles.len() > 1 {
            panic!("More than 1 doubles found: {:?}", doubles);
        } else if doubles.len() == 1 {
            let score = score(*doubles[0]);
            sum += score;
        }
    }
    println!("sum: {}", sum);
}

#[allow(dead_code)]
fn part2() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;

    let mut group: Vec<Vec<u8>> = Vec::new();

    for (i, rline) in stdin.lock().lines().enumerate() {
        group.push(rline.unwrap().into_bytes());
        if i % 3 == 2 {
            let b0: HashSet<u8> = group[0].clone().into_iter().collect();
            let b1: HashSet<u8> = group[1].clone().into_iter().collect();
            let doubles: HashSet<&u8> = group[2]
                .iter()
                .filter(|e| b0.contains(e) && b1.contains(e))
                .collect();
            let doubles: Vec<&u8> = doubles.into_iter().collect();
            if doubles.len() > 1 {
                panic!("More than 1 doubles found: {:?}", doubles);
            } else if doubles.len() == 1 {
                let score = score(*doubles[0]);
                sum += score;
            }

            group = Vec::new();
        }
    }
    println!("sum: {}", sum);
}

fn main() {
    part2();
}
