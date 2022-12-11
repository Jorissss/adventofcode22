use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_size(lines: &Vec<String>) -> HashMap<String, u32> {
    let mut dirsizes: HashMap<String, u32> = HashMap::new();
    let mut in_dirs: Vec<&str> = vec![];
    for line in lines.iter() {
        let cmd: Vec<&str> = line.split_whitespace().collect();

        if cmd.len() == 0 {
            // to deal with empty lines (particularly the last line :))
            continue;
        } else if cmd[0] == "$" && cmd[1] == "cd" && cmd[2] == ".." {
            in_dirs.pop();
        } else if cmd[0] == "$" && cmd[1] == "cd" {
            in_dirs.push(cmd[2]);
        } else if let Ok(size) = cmd[0].parse::<u32>() {
            for (i, _) in in_dirs.iter().enumerate() {
                let entry = dirsizes.entry(in_dirs[..i + 1].join("/")).or_insert(0);
                *entry += size;
            }
        }
    }

    return dirsizes;
}

fn find_free_space(
    dirsizes: &Vec<u32>,
    space_total: u32,
    space_used: u32,
    space_needed: u32,
) -> u32 {
    let space_free = space_total - space_used;
    let space_to_free = space_needed - space_free;

    let mut candidate_folders: Vec<&u32> =
        dirsizes.iter().filter(|x| **x >= space_to_free).collect();
    candidate_folders.sort();
    return *candidate_folders[0];
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut lines_str: Vec<String> = vec![];
    for line in lines {
        let rline = line.unwrap();
        lines_str.push(rline);
    }
    let dirsizes_map = count_size(&lines_str);
    let space_used = *dirsizes_map.get("/").unwrap();
    let dirsizes: Vec<u32> = dirsizes_map.into_values().collect();
    let mut sum = 0;
    for size in dirsizes.iter() {
        if *size <= 100000 {
            sum += size;
        }
    }
    println!("part 1: {}", sum);
    let freed_space = find_free_space(&dirsizes, 70000000, space_used, 30000000);
    println!("part 2: {}", freed_space);
}
