use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn find_marker(input: &Vec<u8>, start: usize, marker_len: usize) -> usize {
    let mut marker: usize = 256;
    let mut queue: VecDeque<char> = VecDeque::with_capacity(marker_len);
    for (mut i, ch) in input[start..].iter().enumerate() {
        i += start;
        println!("{}\t{}\t{:?}", i, *ch as char, queue);
        let in_queue = queue.contains(&(*ch as char));
        if queue.len() >= marker_len {
            marker = i;
            break;
        }
        if in_queue {
            while queue.pop_front().unwrap() != *ch as char {}
        }
        queue.push_back(*ch as char);
    }
    return marker;
}

fn run() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().next().unwrap().unwrap().into_bytes();
    let package_marker = find_marker(&input, 0, 4);
    let message_marker = find_marker(&input, 0, 14);
    println!("package_marker: {:?}", package_marker);
    println!("message_marker: {:?}", message_marker);
}

fn main() {
    run();
}
