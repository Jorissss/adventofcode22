use std::{
    collections::HashSet,
    io::{self, BufRead},
    iter,
};

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Move {
    fn from_string(string: &str) -> Move {
        let cmd: Vec<&str> = string.split_whitespace().collect();
        match cmd[0] {
            "U" => Move::Up(cmd[1].parse::<u32>().unwrap()),
            "D" => Move::Down(cmd[1].parse::<u32>().unwrap()),
            "L" => Move::Left(cmd[1].parse::<u32>().unwrap()),
            "R" => Move::Right(cmd[1].parse::<u32>().unwrap()),
            _ => panic!("not a valid move input"),
        }
    }
}

impl Pos {
    fn move_y(&self, n: i32) -> Self {
        Pos {
            y: self.y + n,
            ..*self
        }
    }
    fn move_x(&self, n: i32) -> Self {
        Pos {
            x: self.x + n,
            ..*self
        }
    }

    fn move_xy(&self, xy: (i32, i32)) -> Self {
        Pos {
            x: self.x + xy.0,
            y: self.y + xy.1,
        }
    }

    fn move_pos(&self, mv: Move) -> Self {
        match mv {
            Move::Up(n) => self.move_y(n as i32),
            Move::Down(n) => self.move_y(-(n as i32)),
            Move::Left(n) => self.move_x(-(n as i32)),
            Move::Right(n) => self.move_x(n as i32),
        }
    }

    fn difference(&self, pos: &Pos) -> (i32, i32) {
        (-(self.x - pos.x), -(self.y - pos.y))
    }

    fn move_tail(&self, headpos: &Pos) -> Self {
        let diff = self.difference(headpos);
        if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
            return self.clone();
        }

        return Pos {
            x: self.x
                + if diff.0 != 0 {
                    diff.0 / diff.0.abs()
                } else {
                    0
                },
            y: self.y
                + if diff.1 != 0 {
                    diff.1 / diff.1.abs()
                } else {
                    0
                },
        };
    }
}

fn move_vec(visited: &mut HashSet<Pos>, head: &Pos, tail: &Pos, vec: (i32, i32)) -> (Pos, Pos) {
    let steps = (vec.0 + vec.1).abs();
    let mv_vec = (vec.0 / steps, vec.1 / steps);
    let mut head_pos = head.clone();
    let mut tail_pos = tail.clone();
    for _ in 0..steps {
        head_pos = head_pos.move_xy(mv_vec);
        tail_pos = tail_pos.move_tail(&head_pos);
        visited.insert(tail_pos);
    }

    (head_pos, tail_pos)
}

fn move_both(visited: &mut HashSet<Pos>, mv: &Move, head: &Pos, tail: &Pos) -> (Pos, Pos) {
    match mv {
        Move::Up(n) => move_vec(visited, head, tail, (0, -(*n as i32))),
        Move::Down(n) => move_vec(visited, head, tail, (0, *n as i32)),
        Move::Left(n) => move_vec(visited, head, tail, (-(*n as i32), 0)),
        Move::Right(n) => move_vec(visited, head, tail, (*n as i32, 0)),
    }
}

fn move_vec_all(visited: &mut HashSet<Pos>, snake: &mut Vec<Pos>, vec: (i32, i32)) {
    let steps = (vec.0 + vec.1).abs();
    let mv_vec = (vec.0 / steps, vec.1 / steps);
    let mut head_pos = snake[0].clone();
    let mut tail_pos = snake[0].clone();
    for _ in 0..steps {
        snake[0] = snake[0].move_xy(mv_vec);
        for i in 1..10 {
            snake[i] = snake[i].move_tail(&snake[i - 1]);
        }
        visited.insert(snake[9].clone());
    }
}

fn move_all(visited: &mut HashSet<Pos>, snake: &mut Vec<Pos>, mv: &Move) {
    match mv {
        Move::Up(n) => move_vec_all(visited, snake, (0, -(*n as i32))),
        Move::Down(n) => move_vec_all(visited, snake, (0, *n as i32)),
        Move::Left(n) => move_vec_all(visited, snake, (-(*n as i32), 0)),
        Move::Right(n) => move_vec_all(visited, snake, (*n as i32, 0)),
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut headpos = Pos { x: 0, y: 0 };
    let mut tailpos = Pos { x: 0, y: 0 };
    let mut visited: HashSet<Pos> = HashSet::new();
    for line in lines {
        let mv: Move = Move::from_string(line);
        (headpos, tailpos) = move_both(&mut visited, &mv, &headpos, &tailpos);
        visited.insert(tailpos);
    }
    return visited.len() as u32;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut snake: Vec<Pos> = iter::repeat(Pos { x: 0, y: 0 }).take(10).collect();
    let mut visited: HashSet<Pos> = HashSet::new();
    for line in lines {
        let mv: Move = Move::from_string(line);
        move_all(&mut visited, &mut snake, &mv);
    }
    return visited.len() as u32;
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
