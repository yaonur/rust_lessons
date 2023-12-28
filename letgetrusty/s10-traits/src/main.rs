pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!(
    //         "{}, by {} ({})",
    //         &self.headline, &self.author, &self.content
    //     )
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", &self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", &self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let some_tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let some_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", some_article.summarize());
    println!("1 new tweet: {}", some_tweet.summarize());
    println!("tweet by {}", some_tweet.summarize_author());
    println!("article by {}", some_article.summarize_author());
    notify(&some_tweet);
}
