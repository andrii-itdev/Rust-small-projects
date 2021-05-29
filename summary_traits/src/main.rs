
pub mod aggregator;
pub mod structures;

use std::{fmt::Display, usize};

use aggregator::Summary as Summary;

pub fn notify(item : &(impl Summary + Display) ){
    //println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item);
}

// pub fn notify<T : Summary + Display>(item : &T) {
//     //println!("Breaking news! {}", item.summarize());
//     println!("Breaking news! {}", item);
// }

// fn returns_summarizable() -> impl Summary {
//     structures::Tweet {
//         username : String::from("horse_ebooks"),
//         content : String::from("of course, as you probably already know, people"),
//         reply : false,
//         reptweet : false
//     }
// }

// Playing with strings
trait EasyString {
    fn nth(&self, i : usize) -> char;
}

impl EasyString for String {
    fn nth(&self, i : usize) -> char 
    {
        self.chars().nth(i).unwrap()
    }
}

// fn treat_string(mystring : impl EasyString) {
//     println!("mystring: {}", mystring.nth(5));
// }

fn main() {
    let tweet = structures::Tweet {
        username : String::from("horse_ebooks"),
        content : String::from("of course, as you probably already know, people"),
        reply : false,
        reptweet : false
    };
    //println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);

    //treat_string(String::from("My string! Hello!"));
}

