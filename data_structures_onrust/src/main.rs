use std::io::Write;
use std::str::FromStr;


struct Point<T> {
    x: T,
    y: T,
}

#[warn(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct PointTwoValues<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwoValues<T, U> {
    fn mixup<V , W>(self, other: PointTwoValues<V, W>) -> PointTwoValues<T, W> {
        PointTwoValues {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct NewsArticleDefault {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
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

impl SummaryDefault for NewsArticleDefault {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
           let t = m;
           m = n;
           n = t;
        }
        m %= n;
    }
    n
}

fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");

    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.is_empty() {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    let number_list_1 = vec![34, 50, 25, 100, 65];
    let largest_1 = largest_i32(&number_list_1);
    println!("The largest number from largest_i32 is {}", largest_1);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point{x: 1.0, y:4.0 };
    println!("integer.x = {}, _integer.y = {}", _integer.x, _integer.y);
    println!("float.x = {}, _float.y = {}", _float.x, _float.y);

    let p1 = PointTwoValues{x:5, y:10.4};
    let p2 = PointTwoValues{x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}. p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet:{}", tweet.summarize());

    let article = NewsArticleDefault {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
    };
    println!("New article available !{}", article.summarize());
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}
