use std::io::{self, BufRead};

#[allow(dead_code)]
fn part1() {
    let stdin = io::stdin();
    let mut part1_sum: u32 = 0;
    let mut part2_sum: u32 = 0;

    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        let elves: Vec<&str> = line.split(',').collect();
        let ranges: Vec<Vec<u32>> = elves
            .iter()
            .map(|v| v.split('-').map(|s| s.parse::<u32>().unwrap()).collect())
            // .map(|v: Vec<u32>| (v[0]..v[1] + 1).collect())
            .collect();
        if (ranges[0][0] >= ranges[1][0] && ranges[0][1] <= ranges[1][1])
            || (ranges[0][0] <= ranges[1][0] && ranges[0][1] >= ranges[1][1])
        {
            part1_sum += 1;
        }
        if ranges[1][1] >= ranges[0][1] && ranges[0][1] >= ranges[1][0]
            || ranges[0][1] >= ranges[1][1] && ranges[1][1] >= ranges[0][0]
        {
            part2_sum += 1;
        }
    }
    println!("part 1: {:?}", part1_sum);
    println!("part 2: {:?}", part2_sum);
}

fn main() {
    part1();
}
