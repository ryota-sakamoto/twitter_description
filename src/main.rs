extern crate scraper;
extern crate reqwest;

use scraper::{Html, Selector};
use std::env;

#[derive(Debug)]
struct Description {
    id: String,
    display_name: String,
    tweet_count: u64,
    follow_count: u64,
    follower_count: u64,
    like_count: u64,
}

fn main() {
    let mut args = env::args();
    args.next();
    while let Some(id) = args.next() {
        let data = get_description(id);
        println!("{:?}", data);
    }
}

fn get_description(id: String) -> Description {
    let mut res = reqwest::get(&format!("https://twitter.com/{}", id)).unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);

    let user_selector = Selector::parse(".ProfileHeaderCard").unwrap();
    let user = document.select(&user_selector).next().unwrap();

    let display_name_selector = Selector::parse(".ProfileHeaderCard-nameLink").unwrap();
    let display_name = user.select(&display_name_selector).next().unwrap().text().next().unwrap();

    let id_selector = Selector::parse(".u-linkComplex-target").unwrap();
    let id = user.select(&id_selector).next().unwrap().text().next().unwrap();

    let count_selector = Selector::parse(".ProfileNav-list .ProfileNav-value").unwrap();
    let mut select = document.select(&count_selector);
    let tweet_count = select.next().unwrap().value().attr("data-count").unwrap().parse().unwrap();
    let follow_count = select.next().unwrap().value().attr("data-count").unwrap().parse().unwrap();
    let follower_count = select.next().unwrap().value().attr("data-count").unwrap().parse().unwrap();
    let like_count = select.next().unwrap().value().attr("data-count").unwrap().parse().unwrap();

    Description {
        id: id.to_string(),
        display_name: display_name.to_string(),
        tweet_count: tweet_count,
        follow_count: follow_count,
        follower_count: follower_count,
        like_count: like_count,
    }
}
