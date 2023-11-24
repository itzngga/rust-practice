use std::fmt::{Debug, format};
use std::iter::Sum;

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        // String::from("read more")
        format!("(Read more from {} ...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
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

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//        return format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         return format!("{}: {}", self.username, self.content);
//     }
//
// }

// impl Summary for NewsArticle {
//
// }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) -> String { // impl trait syntax
    format!("Breaking news! {}", item.summarize())
}

// pub fn notify2<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// pub fn notify3(item: &impl Summary, item2: &impl Summary) {
//
// }
//
// pub fn notify4<T: Summary>(item1: &T, item2: &T) { // type yg sama (trait bound generic)
//
// }

use std::fmt::Display;

// pub fn notify2(item: &(impl Summary + Display)) {

// }

// pub fn notify3<T: Summary + Display>(item: &T){}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//
// }
//
// fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {
//
// }

// closures dan iterator
fn return_summarizable() -> impl Summary {
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("lorem_ipsum"),
        reply: false,
        retweet: false,
    }
}

// fn return_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//                 headline: String::from("Cara baru membuat tweet"),
//                 location: String::from("Lost Angeles"),
//                 author: String::from("rust_lang"),
//                 content: String::from("Lorem ipsum dolor sit amet"),
//         }
//     } else {
//         Tweet{
//             username: String::from("horse_ebooks"),
//             content: String::from("lorem_ipsum"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl <T> Pair <T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Display + std::cmp::PartialOrd> Pair <T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}