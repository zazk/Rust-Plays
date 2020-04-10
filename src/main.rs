#![allow(dead_code)]

mod files;
mod hashmaps;
mod strings;
mod structs;
mod vectors;

enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey("Pressed w".to_string()),
            Direction::Down(_) => Keys::DownKey("Pressed s".to_string()),
            Direction::Left(_) => Keys::LeftKey("Pressed a".to_string()),
            Direction::Right(_) => Keys::RightKey("Pressed d".to_string()),
        }
    }
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let u = Direction::Up(Point { x: 32, y: 54 });
    let k = u.match_direction();
    println!("Direction Matched:{:?} Key:{}", k, k.destruct());
}
