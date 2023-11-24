mod lib;
use crate::lib::{NewsArticle, Summary, Tweet, notify, Pair};

fn main() {
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Lorem Ipsum Dolor Sit Amet"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Cara baru membuat tweet"),
    //     location: String::from("Lost Angeles"),
    //     author: String::from("rust_lang"),
    //     content: String::from("Lorem ipsum dolor sit amet"),
    // };
    //
    // println!("new article available: {}", article.summarize());

    println!("{}", notify(&tweet));

    let p = Pair::new(2, 6);
    println!("{:?}", p.cmp_display());

    let s = 3.to_string();
}
