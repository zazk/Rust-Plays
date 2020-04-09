use std::fmt;

#[derive(Debug)]
struct Object {
    height: u16,
    width: u16,
}

//Methods
impl Object {
    fn area(&self) -> u16 {
        self.height * self.width
    }
    fn new(width: u16, height: u16) -> Object {
        Object { width, height }
    }
}

// Common
impl Object {
    fn show(&self) {
        println!(
            "Struct Another Object {}x{} area:{:?}",
            self.height,
            self.width,
            self.area()
        );
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{}) and area: {}",
            self.width,
            self.height,
            self.area()
        )
    }
}

pub fn main() {
    // Structs
    println!("\nStructs =========================");
    let obj: Object = Object {
        width: 32,
        height: 543,
    };
    let another_obj = Object::new(12, 24);
    obj.show();
    another_obj.show();

    println!("Object Formatterd: {}", obj);
    println!("Object Debug: {:?}", obj);
    println!("Object Pretty: {:#?}", another_obj);
}
