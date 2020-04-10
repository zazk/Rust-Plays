use std::collections::HashMap;

pub fn main() {
    println!("\nHashMap ===========================================");
    let mut hm = HashMap::new();
    hm.insert("random".to_string(), 12);
    hm.insert(String::from("numerical"), 16);
    for (k, v) in &hm {
        println!("My HashMap: {}:{}", k, v);
    }
    match hm.get(&String::from("No key")) {
        Some(&n) => println!("Matched {}", n),
        _ => println!("Not matched"),
    }
    hm.remove(&"numerical".to_string());

    let s = Some("c");
    match s {
        Some(i) => println!("Matched: {}", i),
        _ => {}
    }

    if let Some(i) = s {
        println!("Matched: {}", i)
    } else {
        {}
    }

    let mut so = Some(0);
    loop {
        match so {
            Some(i) => {
                if i > 19 {
                    println!("Quit");
                    so = None;
                } else {
                    println!("{}", i);
                    so = Some(i + 2)
                }
            }
            _ => {
                break;
            }
        }
    }

    let mut son = Some(0);

    while let Some(i) = son {
        if i > 19 {
            println!("Quit");
            son = None;
        } else {
            println!("{}", i);
            son = Some(i + 2)
        }
    }

    let f = 24.4321_f32;
    let i = f as u8;
    let c = i as char;

    println!("Casting {},{},{}--", f, i, c);
    println!("Weird chars:{} {} {}", 255 as char, 44 as char, 45 as char);
}
