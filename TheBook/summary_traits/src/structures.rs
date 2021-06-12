
use std::{
    fmt::{
        self, 
        Display, 
        Formatter
    }
};

use crate::aggregator::Summary as Summary;

#[derive(Debug)]
pub struct NewsArticle {
    pub headlines: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {} ({})", self.headlines, self.location, self.author)
    // }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub reptweet : bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}


// Display

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tweet: ({}; {}; {}; {})", self.username, self.content, self.reply, self.reptweet)
    }
}
