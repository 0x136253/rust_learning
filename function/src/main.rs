fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, '😀');
    express_test();
    println!("The value of five() is: {}", five());
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}

fn express_test(){
    let x = 5;
    let y = {
        let x = 3;
        x+1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32{
    5
}