use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn run() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().next().unwrap().unwrap().into_bytes();
    let mut marker: usize = 256;
    let mut queue: Vec<u8> = Vec::with_capacity(5);
    for (i, ch) in input.iter().enumerate() {
        if queue.contains(ch) {}
    }
    println!("marker: {:?}", marker);
}

fn main() {
    run();
}
