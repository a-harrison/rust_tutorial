// Traits are like Interfaces, used to define common behvior across various types
// Code below defines a 'Summary' trait for various text media
// By default an implemenation will override the implemenation below
// This implemenation will only be used if it is defined as empty
pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("this author")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Disabling means it will use the default implemenation
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// Use the default implemenation
impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Doesn't imeplement Summary
pub struct InstagramPost {
    pub username: String,
    pub likes: i32,
    pub message: String
}

// Implement a function that takes a Trat as a parameter
// Uses 'Impl Trait' syntax
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Define it using 'Trait Bound Syntax'
// pub fn notify<T: Summary>(item: T) {
//     ...
// }

pub fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    let _post = InstagramPost {
        username: String::from("kittenxlady"),
        likes: 1000,
        message: String::from("TNR your cats!"),
    };

    notify(tweet);
    notify(article);

    // Won't compile if uncommented; InstagramPost doesn't implement Summary
    // notify(post);
}
