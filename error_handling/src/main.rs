use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}",e),
            }
            other_error => panic!("Problem opening the file: {:?}",other_error),
        }
    };

    println!("{:?}",f);
}
