#[allow(dead_code)]
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

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(i32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(ref r) => 3.14 * r * r,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Rectangle { width, height } => (width * height) as f64,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

pub fn main() {
    let c = Shape::Circle(3.5);
    let s = Shape::Square(34);
    let r = Shape::Rectangle {
        width: 76,
        height: 65,
    };

    let ac = c.area();
    println!("Circle Area:{}", ac);
    let asq = s.area();
    println!("Square Area:{}", asq);
    let ar = r.area();
    println!("Rectangle Area:{}", ar);
    let u = Direction::Up(Point { x: 32, y: 54 });
    let k = u.match_direction();
    println!("Direction Matched:{:?} Key:{}", k, k.destruct());

    let div = division(44.0, 3.0);
    match div {
        Some(x) => {
            println!("Result->{}", x);
            println!("Result with 2 decimals->{:.2}", x);
        }
        None => println!("Can't divide by 0"),
    }
}
