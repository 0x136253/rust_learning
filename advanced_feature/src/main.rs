extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
