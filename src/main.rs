mod strings;
mod structs;
use strings::*;

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    strings();
    structs::main();

    // Vectors
    println!("\nVectors =========================");
    let vectory = vec![1, 2, 3, 4];
    for i in &vectory {
        println!("My item in vector {}", i);
    }
    let mut vectory_class = Vec::new();
    vectory_class.push(6);
    vectory_class.push(7);
    vectory_class.push(8);
    vectory_class.push(9);
    println!(
        "vector_class:{:?} length:{} capacity:{} ",
        vectory_class,
        vectory_class.len(),
        vectory_class.capacity()
    );
    vectory_class.push(11);
    println!(
        "vector_class:{:?} length:{} capacity:{} ",
        vectory_class,
        vectory_class.len(),
        vectory_class.capacity()
    );

    println!("vector_class.pop():{:?}", vectory_class.pop());

    let mut v: Vec<i32> = Vec::new();
    for i in &v {
        println!("Empty vector:{}", i);
    }

    println!("v:{:?} length:{} capacity:{} ", v, v.len(), v.capacity());
    println!("v.pop():{:?}", v.pop());

    let r = vec![
        Example::Int(22),
        Example::Float(12.2),
        Example::Text("Hi Enum in vector".to_string()),
    ];
    println!("Vecto in Enum r:{:?}", r);
}
