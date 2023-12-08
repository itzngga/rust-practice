// fn main() {
//     let novel = String::from("lorem isum. some lorem...");
//     let first_sentence = novel.split('.').next().expect("Colud not find .");
//
//    let i = ImportantExcerpt{
//        part: first_sentence,
//    };
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x
//     } else {
//         return y
//     }
// }
//
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
//     return x
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// impl<'a> ImportantExcerpt <'a> {
//     fn level(&self) -> i32 {
//         4
//     }
// }

// impl <'a> ImportantExcerpt <'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("lorem isum. some lorem...");
//     let first_sentence = novel.split('.').next().expect("Colud not find .");
//
//     let i = ImportantExcerpt{
//         part: first_sentence,
//     };
//
//     let s: &'static str = "i am static lifetime";
// }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2, "Today is special");
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest<'a, T>(
    x: &'a str,
    y: &'a str,
    txt: T,
) -> &'a str where T: Display {
    println!("Announcement! {}", txt);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}