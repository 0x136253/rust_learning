pub trait Summary {
    fn summarize(&self) -> String;
}

struct Point<T> {
    x: T,
    y: T,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct ImportantExcert<'a> {
    head: &'a str,
    part:&'a str,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    println!("Hello, world!");
    let wont_work = Point {x:5,y:5};
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(tweet);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
        result = ImportantExcert {
            head:string1.as_str(),
            part:string2.as_str(),
        };
        // println!("The longest string is {}",result);
        println!("The longest string is {} {}",result.head,result.part);
    }
    // result = "aaa";
    // println!("The longest string is {} {}",result.head,result.part);
}


pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn longest<'a> (x: &'a str, y:&'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}