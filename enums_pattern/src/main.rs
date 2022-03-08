#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8{
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!",state);
                25
            },
        }
    }
}


impl Message {
    fn call(&self) {
        println!("call Message:{:?}",self);
    }
}

fn main() {
    println!("Hello, world!");

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}",home);
    println!("{:?}",loopback);

    let msg = Message::Move {x:12,y:23};

    println!("{:?}",msg);
    msg.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{:?},{:?},{:?}",some_number,some_string,absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(7);

    let sum = x + y.expect("aaa");

    println!("{}",sum);

    let coin = Coin::Quarter(UsState::Alabama);

    println!("{}",coin.value_in_cents());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        other_object => println!("other:{}",other_object),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        Some(0) => println!("three:0"),
        _ => (),
    }

    if let Some(x) = some_u8_value {
        println!("Three:{}",x);
    };

    // if let some_u8_value = Some(3) {
    //     println!("Three");
    // };

    let x = 0u8;

    println!("{}",x);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
