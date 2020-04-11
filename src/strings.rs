pub fn main() {
    // Vectors
    println!("\nStrings =========================");
    let condition: bool = true;
    if condition {
        println!("Hello there");
    } else {
        println!("Hello negative");
    }
    let number: u8 = 250;
    let some_number: i32 = 21;
    let some_string: &str = "Lorep Ipsum Wannabe";
    let names = ["One", "Xavier", "Logan"];
    let point = (32, 132); //(i32, i32)
    let point_tag = ("A", 32, 32); //(&str, i32,i32)
    let (head, tail) = "Hi Hell".split_at(3);
    let exclusive_string = &some_string[0..8];
    let inclusive_string = &some_string[0..=8];

    let pro = &process();
    let pro_string = process_string();
    println!("My pro value is:{} Pro {}", pro, pro_string);
    println!("{} its another String", some_string);
    println!("it's a slice exclusive of String {} ", exclusive_string);
    println!("it's a slice inclusive of String {} ", inclusive_string);
    println!("My number {}", number);
    println!("Array of Names {:?}", names);
    println!("My number {}", some_number);
    println!("My point {} {}", point.0, point.1);
    println!("My touple {} {}", head, tail);
    println!("My point_tag {:?}", point_tag);

    // match
    match number {
        1 => println!("One"),
        2..=12 => println!("Undecimals"),
        13..=100 => println!("teens"),
        _ => println!("No acceptable cases"),
    }
    match point {
        (x, 32) => println!("My number x is {}", x),
        (32, y) => println!("My number y is {}", y),
        _ => println!("No match"),
    }
    match number {
        n @ 1..=10 => println!("Minor: {}", n),
        n @ 11..=255 => println!("Mayor: {}", n),
        _ => println!("No match"),
    }
}

fn process() -> &'static str {
    let greeting = "Whats!";
    println!("{}!", greeting);
    greeting
}

fn process_string() -> String {
    let another_greeting = "Whats!".to_string();
    let greeting = String::from("Whats!");
    println!(
        "String:from ->{}! to_string ->{}!",
        greeting, another_greeting
    );
    another_greeting
}
