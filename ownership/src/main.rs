#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle{width:size, height:size}
    }

    fn area(&self) -> u32{
        // self.width+=1;
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height>other.width
    }
}

fn main() {
    // let rect1 = Rectangle { width: 30, height: 50 };
    // let rect2 = Rectangle { width: 10, height: 40 };
    // let rect3 = Rectangle { width: 60, height: 45 };
    // let sq = Rectangle::square(12);

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    // println!("{}",r1);

    let r4 =&mut s;
    println!("{}",r4);

}

