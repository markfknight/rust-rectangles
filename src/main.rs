
fn main() {
    let tweet = Tweet {
        username: String::from("markfknight"),
        content: String::from("I ROCK!"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("test"),
        location: String::from("test"),
        author: String::from("test"),
        content: String::from("test"),
    };

    notify(news);

        let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub trait Summary {
    fn summarise_author(&self) -> String;

    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T>(item: T)
    where T: Summary
{
    println!("Breaking news! {}", item.summarise());
}

pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
