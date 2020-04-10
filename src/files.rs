use std::fs::File;
pub fn main() {
    let f = File::open("test.txt");

    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There is a problem oppening the file {:?}", error);
        }
    };
}
