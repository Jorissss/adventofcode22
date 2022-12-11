use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_size(lines: &Vec<String>) -> HashMap<&str, u32> {
    let mut dirsizes: HashMap<&str, u32> = HashMap::new();
    let mut in_dirs: Vec<&str> = vec![];
    let mut count = 0;
    for line in lines.iter() {
        let cmd: Vec<&str> = line.split_whitespace().collect();

        if cmd.len() == 0 {
            continue;
        } else if cmd[0] == "$" && cmd[1] == "cd" && cmd[2] == ".." {
            in_dirs.pop();
        } else if cmd[0] == "$" && cmd[1] == "cd" {
            dirsizes.entry(cmd[2]).or_insert(0);
            in_dirs.push(cmd[2]);
        } else if cmd[0] == "dir" {
            dirsizes.entry(cmd[1]).or_insert(0);
        } else if let Ok(size) = cmd[0].parse::<u32>() {
            for dir in in_dirs.iter() {
                let entry = dirsizes.entry(dir).or_insert(0);
                *entry += size;
            }
        }
        count += 1;
    }

    println!("{:?}", count);
    println!("{:?}", in_dirs);

    return dirsizes;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut lines_str: Vec<String> = vec![];
    for line in lines {
        let rline = line.unwrap();
        lines_str.push(rline);
    }
    println!("lines: {}", lines_str.len());
    let dirsizes = count_size(&lines_str);
    println!("{:?}", dirsizes);
    let mut sum = 0;
    for (_, size) in dirsizes.iter() {
        if *size < 100000 {
            sum += size;
        }
    }
    println!("{}", sum);
}
