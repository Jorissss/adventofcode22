use std::io::{self, BufRead};

fn parse_towers(input: Vec<&String>) -> Vec<Vec<u32>> {
    let last_row = &input[input.len() - 1];
    let row_numbers = last_row.split_whitespace().map(|n| n.parse::<u8>());
    println!("{:?}", row_numbers);
    return Vec::new();
}

fn main() {
    let mut towersinput: Vec<&String> = Vec::new();

    let stdin = io::stdin();
    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        if line.len() == 0 {
            break;
        }
        towersinput.push(&line);
        println!("{:?}", line);
    }

    let mut towers: Vec<Vec<u32>> = parse_towers(towersinput);

    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        let line = line.as_bytes();
        println!("{:?}", line);
    }
}
