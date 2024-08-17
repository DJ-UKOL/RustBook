use crate::aggregator::{Summary, Tweet};

pub fn traits() {
    let tweet = Tweet {
        username: "horse_ebook".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

