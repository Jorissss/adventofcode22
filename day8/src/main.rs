use std::io::{self, BufRead};

fn read_input() -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![];
    let lines = io::stdin().lock().lines();
    for line in lines {
        grid.push(line.unwrap().into_bytes().iter().map(|x| x - 48).collect());
    }

    return grid;
}

fn is_visible(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let mut visible_dirs: Vec<bool> = vec![true, true, true, true];
    if i == 0 || j == 0 {
        return true;
    }
    if i == grid.len() - 1 || j == grid[0].len() - 1 {
        return true;
    }
    for (i_, _) in grid.iter().enumerate() {
        for (j_, _) in grid[i_].iter().enumerate() {
            if i_ == i && j_ < j && grid[i_][j_] >= grid[i][j] {
                visible_dirs[0] = false;
            }
            if i_ == i && j_ > j && grid[i_][j_] >= grid[i][j] {
                visible_dirs[1] = false;
            }
            if j_ == j && i_ > i && grid[i_][j_] >= grid[i][j] {
                visible_dirs[2] = false;
            }
            if j_ == j && i_ < i && grid[i_][j_] >= grid[i][j] {
                visible_dirs[3] = false;
            }
        }
    }

    return visible_dirs.iter().fold(false, |x, y| x || *y);
}

fn count_visible(grid: &Vec<Vec<u8>>) -> u32 {
    let mut count: u32 = 0;
    for (i, _) in grid.iter().enumerate() {
        for (j, _) in grid.iter().enumerate() {
            if is_visible(grid, i, j) {
                count += 1;
            }
        }
    }
    return count;
}

fn compute_scenic_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    let w = grid[0].len();
    let h = grid.len();
    if i == 0 || j == 0 {
        return 0;
    }
    if i == h - 1 || j == w - 1 {
        return 0;
    }
    println!("i: {} j: {}", i, j);
    let mut west_score = 1;
    if !(j == 1) {
        for j_ in (1..j).rev() {
            if grid[i][j_] < grid[i][j] {
                west_score += 1;
            } else {
                break;
            }
        }
    }
    let mut east_score = 1;
    if !(j == w - 2) {
        for j_ in j + 1..w - 1 {
            if grid[i][j_] < grid[i][j] {
                east_score += 1;
            } else {
                break;
            }
        }
    }
    let mut north_score = 1;
    if !(i == 1) {
        for i_ in (1..i).rev() {
            if grid[i_][j] < grid[i][j] {
                north_score += 1;
            } else {
                break;
            }
        }
    }
    let mut south_score = 1;
    if !(i == h - 2) {
        for i_ in i + 1..h - 1 {
            if grid[i_][j] < grid[i][j] {
                south_score += 1;
            } else {
                break;
            }
        }
    }
    if i == 3 && j == 2 {
        println!("west score: {}", west_score);
        println!("east score: {}", east_score);
        println!("north score: {}", north_score);
        println!("south score: {}", south_score);
    }

    return west_score * east_score * north_score * south_score;
}

fn compute_best_scenic_score(grid: &Vec<Vec<u8>>) -> u32 {
    let mut best: u32 = 0;
    for (i, _) in grid.iter().enumerate() {
        for (j, _) in grid.iter().enumerate() {
            let score = compute_scenic_score(grid, i, j);
            if score > best {
                best = score;
                println!("i: {}, j: {}, score: {}", i, j, score);
            }
        }
    }
    return best;
}

fn main() {
    let grid = read_input();
    println!("part 1: {:?}", count_visible(&grid));
    println!("part 2: {:?}", compute_best_scenic_score(&grid));
}
