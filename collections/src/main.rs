use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();

    // let v = vec![1,2,3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s3 = format!("{}+{}",s1,s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let hello = String::from("नमस्ते");
    let len = hello.len();
    let hello_array = hello.as_bytes();
    // let answer = &hello[0];
    println!("{:?}",hello_array);

    for c in hello.chars() {
        println!("{}",c);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
