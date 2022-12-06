use std::{
    io::{self, BufRead},
    vec,
};

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn print_tower(tower: &Vec<u8>) {
    for e in tower {
        print!("{}", *e as char);
    }
    print!("\n");
}

fn print_towers(towers: &Vec<Vec<u8>>) {
    for tower in towers {
        print_tower(tower);
    }
}

fn parse_towers(mut input: Vec<String>) -> Vec<Vec<u8>> {
    let mut towers: Vec<Vec<u8>> = vec![];
    let last_row = &input[input.len() - 1];
    let row_numbers: Vec<u8> = last_row
        .split_whitespace()
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    for _i in row_numbers {
        towers.push(vec![]);
    }
    input.reverse();

    for line in input[1..].iter() {
        for (i, j) in (1..line.len()).step_by(4).enumerate() {
            if line.as_bytes()[j] != 32 {
                towers[i].push(line.as_bytes()[j]);
            }
        }
    }

    return towers;
}

fn parse_move(moveinput: &String) -> Move {
    let moveinput: Vec<&str> = moveinput.split_whitespace().collect();
    let nums: Vec<usize> = moveinput
        .iter()
        .filter(|&&w| {
            w.to_owned()
                .chars()
                .map(|c| c.is_numeric())
                .fold(true, |acc, e| acc && e)
        })
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    return Move {
        amount: nums[0],
        from: nums[1],
        to: nums[2],
    };
}

fn parse_moves(movesinput: Vec<String>) -> Vec<Move> {
    return movesinput.clone().iter().map(parse_move).collect();
}

fn process_moves(mut towers: Vec<Vec<u8>>, moves: Vec<Move>) -> String {
    for Move { amount, from, to } in moves {
        for _i in 0..amount {
            let value = towers[from - 1].pop().unwrap();
            towers[to - 1].push(value);
        }
    }
    let mut message = "".to_owned();
    for mut tower in towers {
        if tower.len() > 0 {
            message.push(tower.pop().unwrap() as char);
        }
    }
    return message;
}

fn process_moves_part2(mut towers: Vec<Vec<u8>>, moves: Vec<Move>) -> String {
    for Move { amount, from, to } in moves {
        let mut to_move: Vec<u8> = vec![];
        for _i in 0..amount {
            let value = towers[from - 1].pop().unwrap();
            to_move.push(value);
        }
        to_move.reverse();
        for value in to_move {
            towers[to - 1].push(value);
        }
    }
    let mut message = "".to_owned();
    for mut tower in towers {
        if tower.len() > 0 {
            message.push(tower.pop().unwrap() as char);
        }
    }
    return message;
}

fn main() {
    let mut towersinput: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        if line.len() == 0 {
            break;
        }
        towersinput.push(line.clone());
    }

    let mut towers: Vec<Vec<u8>> = parse_towers(towersinput);
    print_towers(&towers);

    let mut movesinput: Vec<String> = vec![];

    for rline in stdin.lock().lines() {
        let line = rline.unwrap();
        if line.len() == 0 {
            break;
        }
        movesinput.push(line.clone());
    }

    let mut moves: Vec<Move> = parse_moves(movesinput);

    println!(
        "message part 1: {}",
        process_moves(towers.clone(), moves.clone())
    );
    println!("message part 2: {}", process_moves_part2(towers, moves));
}
