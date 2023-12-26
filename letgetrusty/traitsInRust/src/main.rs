pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self)->String{
        format!("{}", self.author)
    }
    // fn summarize(&self)->String{
    //     format!("{}, by {} ({})", self.headline, self.author, self.content)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("{}", self.username)
    }
    fn summarize(&self)->String{
        format!("{} tweeted: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

fn returns_summarizable() -> impl Summary{
    Tweet {
        username: String::from("returner function"),
        content: String::from("Some tweets"),
        reply: false,
        retweet: false,
    }
}

pub fn notify(item: impl Summary){
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let tweet = Tweet{
        username: String::from("John"),
        content: String::from("Some content"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle{
        author: String::from("John"),
        headline: String::from("Some headline"),
        content: String::from("Some content"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    notify(tweet);
    println!("{}",returns_summarizable().summarize());
}
